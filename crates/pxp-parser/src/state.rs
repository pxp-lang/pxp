use std::collections::{HashMap, VecDeque};

use pxp_ast::*;
use pxp_diagnostics::{Diagnostic, Severity};
use pxp_lexer::stream::TokenStream;
use pxp_span::Span;
use pxp_symbol::{Symbol, SymbolTable};
use pxp_token::{Token, TokenKind};

use crate::{internal::identifiers::is_soft_reserved_identifier, ParserDiagnostic};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NamespaceType {
    Braced,
    Unbraced,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scope {
    Namespace(Symbol),
    BracedNamespace(Option<Symbol>),
}

#[derive(Debug)]
pub struct State<'a, 'b> {
    pub stack: VecDeque<Scope>,
    pub stream: &'a mut TokenStream<'a>,
    pub symbol_table: &'b mut SymbolTable,
    pub attributes: Vec<AttributeGroup>,
    pub namespace_type: Option<NamespaceType>,
    pub namespace: Option<Symbol>,
    pub diagnostics: Vec<Diagnostic<ParserDiagnostic>>,
    pub imports: HashMap<UseKind, HashMap<Symbol, Symbol>>,
    id: u32,
}

impl<'a, 'b> State<'a, 'b> {
    pub fn new(tokens: &'a mut TokenStream<'a>, symbol_table: &'b mut SymbolTable) -> Self {
        let mut imports = HashMap::new();
        imports.insert(UseKind::Normal, HashMap::new());
        imports.insert(UseKind::Function, HashMap::new());
        imports.insert(UseKind::Const, HashMap::new());

        Self {
            stack: VecDeque::with_capacity(32),
            stream: tokens,
            symbol_table,
            namespace_type: None,
            attributes: vec![],
            diagnostics: vec![],
            namespace: None,
            imports,
            id: 0,
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

    pub fn maybe_resolve_identifier(&mut self, token: Token, kind: UseKind) -> Name {
        let symbol = token.symbol.unwrap();
        let part = match &token.kind {
            TokenKind::Identifier | TokenKind::Enum | TokenKind::From => token.symbol.unwrap(),
            TokenKind::QualifiedIdentifier => {
                let bytestring = self
                    .symbol_table
                    .resolve(token.symbol.unwrap())
                    .unwrap()
                    .to_bytestring();
                let parts = bytestring.split(|c| *c == b'\\').collect::<Vec<_>>();

                self.symbol_table.intern(parts.first().unwrap())
            }
            _ if is_soft_reserved_identifier(&token.kind) => token.symbol.unwrap(),
            _ => unreachable!(),
        };

        let id = self.id();
        let map = self.imports.get(&kind).unwrap();

        // We found an import that matches the first part of the identifier, so we can resolve it.
        if let Some(imported) = map.get(&part) {
            match &token.kind {
                TokenKind::Identifier | TokenKind::From | TokenKind::Enum => {
                    Name::resolved(id, *imported, symbol, token.span)
                }
                TokenKind::QualifiedIdentifier => {
                    // Qualified identifiers might be aliased, so we need to take the full un-aliased import and
                    // concatenate that with everything after the first part of the qualified identifier.
                    let bytestring = self.symbol_table.resolve(symbol).unwrap().to_bytestring();
                    let parts = bytestring.splitn(2, |c| *c == b'\\').collect::<Vec<_>>();
                    let rest = self.symbol_table.intern(parts[1]);
                    let coagulated = self.symbol_table.coagulate(&[*imported, rest], Some(b"\\"));

                    Name::resolved(id, coagulated, symbol, token.span)
                }
                _ => unreachable!(),
            }
        // We didn't find an import, but since we're trying to resolve the name of a class like, we can
        // follow PHP's name resolution rules and just prepend the current namespace.
        //
        // Additionally, if the name we're trying to resolve is qualified, then PHP's name resolution rules say that
        // we should just prepend the current namespace if the import map doesn't contain the first part.
        } else if kind == UseKind::Normal || token.kind == TokenKind::QualifiedIdentifier {
            Name::resolved(id, self.join_with_namespace(symbol), symbol, token.span)
        // Unqualified names in the global namespace can be resolved without any imports, since we can
        // only be referencing something else inside of the global namespace.
        } else if (kind == UseKind::Function || kind == UseKind::Const) && token.kind == TokenKind::Identifier && self.namespace.is_none() {
            Name::resolved(id, symbol, symbol, token.span)
        } else {
            Name::unresolved(id, symbol, token.kind.into(), token.span)
        }
    }

    pub fn add_prefixed_import(
        &mut self,
        kind: &UseKind,
        prefix: Symbol,
        name: Symbol,
        alias: Option<Symbol>,
    ) {
        let coagulated = self.symbol_table.coagulate(&[prefix, name], Some(b"\\"));

        self.add_import(kind, coagulated, alias);
    }

    pub fn add_import(&mut self, kind: &UseKind, name: Symbol, alias: Option<Symbol>) {
        // We first need to check if the alias has been provided, and if not, create a new
        // symbol using the last part of the name.
        let alias = match alias {
            Some(alias) => alias,
            None => {
                let bytestring = self.symbol_table.resolve(name).unwrap().to_bytestring();
                let parts = bytestring.split(|c| *c == b'\\').collect::<Vec<_>>();
                let last = parts.last().unwrap();

                self.symbol_table.intern(last)
            }
        };

        // Then we can insert the import into the hashmap.
        self.imports.get_mut(kind).unwrap().insert(alias, name);
    }

    pub fn strip_leading_namespace_qualifier(&mut self, symbol: Symbol) -> Symbol {
        let bytestring = self.symbol_table.resolve(symbol).unwrap().to_bytestring();

        if bytestring.starts_with(&[b'\\']) {
            self.symbol_table.intern(&bytestring[1..])
        } else {
            symbol
        }
    }

    pub fn join_with_namespace(&mut self, name: Symbol) -> Symbol {
        match self.namespace() {
            Some(Scope::Namespace(namespace)) => self
                .symbol_table
                .coagulate(&[*namespace, name], Some(b"\\")),
            Some(Scope::BracedNamespace(Some(namespace))) => self
                .symbol_table
                .coagulate(&[*namespace, name], Some(b"\\")),
            _ => name,
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
