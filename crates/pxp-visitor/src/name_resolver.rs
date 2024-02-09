#![allow(unused)]

use std::collections::HashMap;

use pxp_ast::{identifiers::SimpleIdentifier, NodeId, Statement};
use pxp_symbol::Symbol;

use crate::Visitor;

#[derive(Debug, Default)]
pub struct NameResolvingVisitor {
    resolver: NameResolver,
    scopes: Vec<Scope>,
}

impl NameResolvingVisitor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resolve(&mut self, ast: &mut [Statement]) -> NameResolver {
        // We always want to enter a "root" scope.
        self.push_scope();
        self.visit(ast);
        self.resolver.clone()
    }

    fn scope(&self) -> &Scope {
        self.scopes.last().unwrap()
    }

    fn scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    fn push_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}

impl Visitor for NameResolvingVisitor {
    fn visit_simple_identifier(&mut self, node: &mut SimpleIdentifier) {
        // 1. If the identifier is fully-qualified, we don't need to do anything
        //    and can simply insert the symbol from the identifier into the map as-is.

        // 2. If the identifier is unqualified, there are a couple of different scenarios:
        //    a. The current scope contains an import that resolves to the identifier, in which case
        //       we can take that full import and insert into the map.
        //    b. The current scope contains an aliased import that resolves to the identifier, in which case
        //       we can take that full import without the alias and insert into the map.
        //    c. The identifier is referencing a named "thing" in the current namespace. If we have an Index,
        //       we can do some additional checks to see if there is anything that matches the potential name in the Index.
        //    d. The identifier references something in the global namespace. We don't need to do anything in this case and
        //       can just put the symbol directly into the map.

        // 3. The identifier is qualified – again, a couple of different scenarios:
        //    a. The first part of the name references an imported namespace or "thing". In which case, we need to concatenate the identifier
        //       and import, then insert them into the map with a new symbol (this requires a mutable SymbolTable).
        //    b. The first part of the name references an aliased import. The same logic as above applies here, but with the "real" name
        //       rather than the alias.
        //    c. We're inside of the global namespace (no namespace or empty namespace {}). In this case, we treat a qualified identifier the
        //       same as a fully-qualified identifier and insert the symbol into the map as-is.
    }
}

/// Internal structure for tracking AST state and current scope.
#[derive(Debug, Default)]
struct Scope {
    namespace: Option<Symbol>,
    uses: HashMap<Symbol, Symbol>,
}

#[derive(Debug, Clone, Default)]
pub struct NameResolver {
    map: HashMap<NodeId, Symbol>,
}

impl NameResolver {
    pub(crate) fn insert(&mut self, id: NodeId, symbol: Symbol) {
        self.map.insert(id, symbol);
    }

    // Not using an `Option<Symbol>` here is a deliberate choice.
    // We should only ever be requesting resolved symbols for nodes that we know can be resolved.
    pub fn resolve(&self, id: NodeId) -> Symbol {
        self.map[&id]
    }
}