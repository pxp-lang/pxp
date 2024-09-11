use std::collections::{HashMap, VecDeque};

use pxp_ast::*;
use pxp_bytestring::ByteString;
use pxp_diagnostics::{Diagnostic, Severity};
use pxp_span::Span;
use pxp_token::{Token, TokenKind};

use crate::{
    internal::identifiers::is_soft_reserved_identifier, ParserDiagnostic,
};

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
    // Unique identifier for each node.
    id: u32,

    // Scope Tracking
    pub stack: VecDeque<Scope>,
    pub imports: HashMap<UseKind, HashMap<ByteString, ByteString>>,
    pub namespace_type: Option<NamespaceType>,
    pub attributes: Vec<AttributeGroup>,
    comments: Vec<Comment>,

    // Token Stream
    tokens: &'a [Token],
    length: usize,
    cursor: usize,

    // Diagnostics
    pub diagnostics: Vec<Diagnostic<ParserDiagnostic>>,
}

impl<'a> State<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        let mut imports = HashMap::new();
        imports.insert(UseKind::Normal, HashMap::new());
        imports.insert(UseKind::Function, HashMap::new());
        imports.insert(UseKind::Const, HashMap::new());

        let mut this = Self {
            stack: VecDeque::with_capacity(32),
            namespace_type: None,
            attributes: vec![],
            imports,
            comments: vec![],
            
            id: 0,

            tokens,
            length: tokens.len(),
            cursor: 0,

            diagnostics: vec![],
        };

        this.collect_comments();

        this
    }

    /// Move cursor to next token.
    ///
    /// Comments are collected.
    pub fn next(&mut self) {
        self.cursor += 1;
        self.collect_comments();
    }

    /// Get current token.
    pub const fn current(&self) -> &'a Token {
        let position = if self.cursor >= self.length {
            self.length - 1
        } else {
            self.cursor
        };

        &self.tokens[position]
    }

    /// Get previous token.
    pub const fn previous(&self) -> &'a Token {
        let position = if self.cursor == 0 { 0 } else { self.cursor - 1 };
        let position = if position >= self.length {
            self.length - 1
        } else {
            position
        };

        &self.tokens[position]
    }

    /// Peek next token.
    ///
    /// All comments are skipped.
    pub const fn peek(&self) -> &'a Token {
        self.peek_nth(1)
    }

    /// Peek nth+1 token.
    ///
    /// All comments are skipped.
    pub const fn lookahead(&self, n: usize) -> &'a Token {
        self.peek_nth(n + 1)
    }

    /// Peek nth token.
    ///
    /// All comments are skipped.
    #[inline(always)]
    const fn peek_nth(&self, n: usize) -> &'a Token {
        let mut cursor = self.cursor + 1;
        let mut target = 1;
        loop {
            if cursor >= self.length {
                return &self.tokens[self.length - 1];
            }

            let current = &self.tokens[cursor];

            if matches!(
                current.kind,
                TokenKind::SingleLineComment
                    | TokenKind::MultiLineComment
                    | TokenKind::HashMarkComment
            ) {
                cursor += 1;
                continue;
            }

            if target == n {
                return current;
            }

            target += 1;
            cursor += 1;
        }
    }

    /// Check if current token is EOF.
    pub fn is_eof(&self) -> bool {
        if self.cursor >= self.length {
            return true;
        }

        self.tokens[self.cursor].kind == TokenKind::Eof
    }

    fn collect_comments(&mut self) {
        loop {
            if self.cursor >= self.length {
                break;
            }

            let current = &self.tokens[self.cursor];

            if !matches!(
                current.kind,
                TokenKind::SingleLineComment
                    | TokenKind::MultiLineComment
                    | TokenKind::HashMarkComment
                    | TokenKind::OpenPhpDoc,
            ) {
                break;
            }

            let id = self.id();
            let comment_id = self.id();

            self.comments.push(match &current {
                Token {
                    kind: TokenKind::SingleLineComment,
                    span,
                    symbol,
                } => Comment {
                    id,
                    span: *span,
                    kind: CommentKind::SingleLine(SingleLineComment {
                        id: comment_id,
                        span: *span,
                        content: symbol.as_ref().unwrap().clone(),
                    }),
                },
                Token {
                    kind: TokenKind::MultiLineComment,
                    span,
                    symbol,
                } => Comment {
                    id,
                    span: *span,
                    kind: CommentKind::MultiLine(MultiLineComment {
                        id: comment_id,
                        span: *span,
                        content: symbol.as_ref().unwrap().clone(),
                    }),
                },
                Token {
                    kind: TokenKind::HashMarkComment,
                    span,
                    symbol,
                } => Comment {
                    id,
                    span: *span,
                    kind: CommentKind::HashMark(HashMarkComment {
                        id: comment_id,
                        span: *span,
                        content: symbol.as_ref().unwrap().clone(),
                    }),
                },
                _ => unreachable!()
            });
            
            self.cursor += 1;
        }
    }

    pub fn comments(&mut self) -> CommentGroup {
        let mut comments = vec![];

        std::mem::swap(&mut self.comments, &mut comments);

        CommentGroup {
            id: self.id(),
            comments: comments.clone()
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
