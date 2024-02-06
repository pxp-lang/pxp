use std::collections::HashMap;

use pxp_ast::{identifiers::SimpleIdentifier, NodeId, Statement};
use pxp_symbol::Symbol;

use crate::Visitor;

#[derive(Debug, Default)]
pub struct NameResolvingVisitor {
    resolver: NameResolver,
}

impl NameResolvingVisitor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resolve(&mut self, ast: &mut [Statement]) -> NameResolver {
        self.visit(ast);
        self.resolver.clone()
    }
}

impl Visitor for NameResolvingVisitor {
    fn visit_simple_identifier(&mut self, node: &mut SimpleIdentifier) {
        
    }
}

/// Internal structure for tracking AST state.
struct State {
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