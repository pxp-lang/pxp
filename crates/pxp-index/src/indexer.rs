use pxp_symbol::{Symbol, SymbolTable};
use pxp_visitor::{walk_braced_namespace, walk_unbraced_namespace, Visitor};
use pxp_ast::{UnbracedNamespace, *};

use crate::Index;

#[derive(Debug, Clone)]
pub struct Indexer {
    index: Index,
    symbol_table: SymbolTable,
    context: IndexerContext,
}

impl Indexer {
    pub fn new(symbol_table: SymbolTable) -> Self {
        Indexer {
            index: Index::new(),
            symbol_table,
            context: IndexerContext::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct IndexerContext {
    namespace: Option<Symbol>,
}

impl Visitor for Indexer {
    fn visit_unbraced_namespace(&mut self, node: &UnbracedNamespace) {
        self.context.namespace = Some(node.name.as_resolved().unwrap().resolved);
        walk_unbraced_namespace(self, node);
        self.context.namespace = None;
    }

    fn visit_braced_namespace(&mut self, node: &BracedNamespace) {
        self.context.namespace = node.name.as_ref().map(|n| n.as_resolved().unwrap().resolved);
        walk_braced_namespace(self, node);
        self.context.namespace = None;
    }
}