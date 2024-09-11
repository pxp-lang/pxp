use std::collections::{HashMap, VecDeque};

use pxp_ast::*;
use pxp_bytestring::ByteString;
use pxp_diagnostics::{Diagnostic, Severity};
use pxp_lexer::stream::TokenStream;
use pxp_span::Span;
use pxp_token::{Token, TokenKind};

use crate::{internal::identifiers::is_soft_reserved_identifier, ParserDiagnostic};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NamespaceType {
    Braced,
    Unbraced,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scope {
    Namespace(ByteString),
    BracedNamespace(Option<ByteString>),
}

#[derive(Debug)]
pub struct State<'a> {
    pub stack: VecDeque<Scope>,
    pub stream: &'a mut TokenStream<'a>,
    pub attributes: Vec<AttributeGroup>,
    pub namespace_type: Option<NamespaceType>,
    pub diagnostics: Vec<Diagnostic<ParserDiagnostic>>,
    pub imports: HashMap<UseKind, HashMap<ByteString, ByteString>>,
    id: u32,
}

impl<'a> State<'a> {
    pub fn new(tokens: &'a mut TokenStream<'a>) -> Self {
        let mut imports = HashMap::new();
        imports.insert(UseKind::Normal, HashMap::new());
        imports.insert(UseKind::Function, HashMap::new());
        imports.insert(UseKind::Const, HashMap::new());

        Self {
            stack: VecDeque::with_capacity(32),
            stream: tokens,
            namespace_type: None,
            attributes: vec![],
            diagnostics: vec![],
            imports,
            id: 0,
        }
    }

    pub fn comments(&mut self) -> CommentGroup {
        let mut comments = vec![];

        std::mem::swap(&mut self.stream.comments, &mut comments);

        CommentGroup {
            id: self.id(),
            comments: comments
                .iter()
                .map(|token| match token {
                    Token {
                        kind: TokenKind::SingleLineComment,
                        span,
                        symbol,
                    } => Comment {
                        id: self.id(),
                        span: *span,
                        kind: CommentKind::SingleLine(SingleLineComment {
                            id: self.id(),
                            span: *span,
                            content: symbol.as_ref().unwrap().clone(),
                        }),
                    },
                    Token {
                        kind: TokenKind::MultiLineComment,
                        span,
                        symbol,
                    } => Comment {
                        id: self.id(),
                        span: *span,
                        kind: CommentKind::MultiLine(MultiLineComment {
                            id: self.id(),
                            span: *span,
                            content: symbol.as_ref().unwrap().clone(),
                        }),
                    },
                    Token {
                        kind: TokenKind::HashMarkComment,
                        span,
                        symbol,
                    } => Comment {
                        id: self.id(),
                        span: *span,
                        kind: CommentKind::HashMark(HashMarkComment {
                            id: self.id(),
                            span: *span,
                            content: symbol.as_ref().unwrap().clone(),
                        }),
                    },
                    // Token {
                    //     kind: TokenKind::DocumentComment,
                    //     span,
                    //     symbol,
                    // } => todo!(),
                    _ => unreachable!(),
                })
                .collect(),
        }
    }

    #[inline(always)]
    pub fn id(&mut self) -> u32 {
        self.id += 1;
        self.id
    }

    pub fn attribute(&mut self, attr: AttributeGroup) {
        self.attributes.push(attr);
    }

    pub fn get_attributes(&mut self) -> Vec<AttributeGroup> {
        let mut attributes = vec![];

        std::mem::swap(&mut self.attributes, &mut attributes);

        attributes
    }

    /// Return the namespace type used in the current state
    ///
    /// The namespace type is retrieve from the last entered
    /// namespace scope.
    ///
    /// Note: even when a namespace scope is exited, the namespace type
    /// is retained, until the next namespace scope is entered.
    pub fn namespace_type(&self) -> Option<&NamespaceType> {
        self.namespace_type.as_ref()
    }

    pub fn namespace(&self) -> Option<&Scope> {
        self.stack.iter().next()
    }

    pub fn maybe_resolve_identifier(&mut self, token: &Token, kind: UseKind) -> Name {
        let symbol = token.symbol.as_ref().unwrap();
        let part = match &token.kind {
            TokenKind::Identifier | TokenKind::Enum | TokenKind::From => {
                token.symbol.as_ref().unwrap().clone()
            }
            TokenKind::QualifiedIdentifier => {
                let bytestring = token.symbol.as_ref().unwrap();
                let parts = bytestring.split(|c| *c == b'\\').collect::<Vec<_>>();

                ByteString::from(parts.first().unwrap().to_vec())
            }
            _ if is_soft_reserved_identifier(&token.kind) => token.symbol.as_ref().unwrap().clone(),
            _ => unreachable!(),
        };

        let id = self.id();
        let map = self.imports.get(&kind).unwrap();

        // We found an import that matches the first part of the identifier, so we can resolve it.
        if let Some(imported) = map.get(&part) {
            match &token.kind {
                TokenKind::Identifier | TokenKind::From | TokenKind::Enum => {
                    Name::resolved(id, imported.clone(), symbol.clone(), token.span)
                }
                TokenKind::QualifiedIdentifier => {
                    // Qualified identifiers might be aliased, so we need to take the full un-aliased import and
                    // concatenate that with everything after the first part of the qualified identifier.
                    let bytestring = symbol.clone();
                    let parts = bytestring.splitn(2, |c| *c == b'\\').collect::<Vec<_>>();
                    let rest = parts[1].to_vec().into();
                    let coagulated = imported.coagulate(&[rest], Some(b"\\"));

                    Name::resolved(id, coagulated, symbol.clone(), token.span)
                }
                _ => unreachable!(),
            }
        // We didn't find an import, but since we're trying to resolve the name of a class like, we can
        // follow PHP's name resolution rules and just prepend the current namespace.
        //
        // Additionally, if the name we're trying to resolve is qualified, then PHP's name resolution rules say that
        // we should just prepend the current namespace if the import map doesn't contain the first part.
        } else if kind == UseKind::Normal || token.kind == TokenKind::QualifiedIdentifier {
            Name::resolved(
                id,
                self.join_with_namespace(symbol),
                symbol.clone(),
                token.span,
            )
        // Unqualified names in the global namespace can be resolved without any imports, since we can
        // only be referencing something else inside of the global namespace.
        } else if (kind == UseKind::Function || kind == UseKind::Const)
            && token.kind == TokenKind::Identifier
            && self.namespace().is_none()
        {
            Name::resolved(id, symbol.clone(), symbol.clone(), token.span)
        } else {
            Name::unresolved(id, symbol.clone(), token.kind.into(), token.span)
        }
    }

    pub fn add_prefixed_import(
        &mut self,
        kind: &UseKind,
        prefix: ByteString,
        name: ByteString,
        alias: Option<ByteString>,
    ) {
        let coagulated = prefix.coagulate(&[name], Some(b"\\"));

        self.add_import(kind, coagulated, alias);
    }

    pub fn add_import(&mut self, kind: &UseKind, name: ByteString, alias: Option<ByteString>) {
        // We first need to check if the alias has been provided, and if not, create a new
        // symbol using the last part of the name.
        let alias = match alias {
            Some(alias) => alias,
            None => {
                let bytestring = name.clone();
                let parts = bytestring.split(|c| *c == b'\\').collect::<Vec<_>>();
                let last = parts.last().unwrap();

                ByteString::new(last.to_vec())
            }
        };

        // Then we can insert the import into the hashmap.
        self.imports.get_mut(kind).unwrap().insert(alias, name);
    }

    pub fn strip_leading_namespace_qualifier(&mut self, symbol: &ByteString) -> ByteString {
        if symbol.starts_with(&[b'\\']) {
            ByteString::from(&symbol[1..])
        } else {
            symbol.clone()
        }
    }

    pub fn join_with_namespace(&mut self, name: &ByteString) -> ByteString {
        match self.namespace() {
            Some(Scope::Namespace(namespace)) => namespace.coagulate(&[name.clone()], Some(b"\\")),
            Some(Scope::BracedNamespace(Some(namespace))) => {
                namespace.coagulate(&[name.clone()], Some(b"\\"))
            }
            _ => name.clone(),
        }
    }

    pub fn previous_scope(&self) -> Option<&Scope> {
        self.stack.get(self.stack.len() - 2)
    }

    pub fn diagnostic(&mut self, kind: ParserDiagnostic, severity: Severity, span: Span) {
        self.diagnostics.push(Diagnostic::new(kind, severity, span));
    }

    pub fn enter(&mut self, scope: Scope) {
        match &scope {
            Scope::Namespace(_) => {
                self.namespace_type = Some(NamespaceType::Unbraced);
            }
            Scope::BracedNamespace(_) => {
                self.namespace_type = Some(NamespaceType::Braced);
            }
        }

        self.stack.push_back(scope);
    }

    pub fn exit(&mut self) {
        self.stack.pop_back();
    }
}
