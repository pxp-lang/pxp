use pxp_symbol::{Symbol, SymbolTable};
use pxp_visitor::{walk_braced_namespace, walk_class_statement, walk_unbraced_namespace, Visitor};
use pxp_ast::{UnbracedNamespace, *};

use crate::{class_like::ClassLike, Index};

#[derive(Debug, Clone)]
pub struct Indexer {
    index: Index,
    context: IndexerContext,
}

impl Indexer {
    pub fn new() -> Self {
        Indexer {
            index: Index::new(),
            context: IndexerContext::default(),
        }
    }

    pub fn index(&mut self, ast: &[Statement]) {
        self.visit(ast);
    }

    pub fn get_index(&self) -> &Index {
        &self.index
    }
}

#[derive(Debug, Clone, Default)]
struct IndexerContext {
    namespace: Option<Symbol>,
    class: Option<ClassLike>,
}

impl IndexerContext {
    fn namespace(&self) -> Option<Symbol> {
        self.namespace
    }

    fn class(&mut self) -> &mut ClassLike {
        self.class.as_mut().unwrap()
    }

    fn set_class(&mut self, class: ClassLike) {
        self.class = Some(class);
    }
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

    fn visit_class_statement(&mut self, node: &ClassStatement) {
        let name = node.name.as_resolved().unwrap();

        self.context.set_class(ClassLike::new(name.resolved, name.original, self.context.namespace()));
        walk_class_statement(self, node);
        
        let class = self.context.class.as_ref().unwrap().clone();

        self.index.add_class(class);
        self.context.class = None;
    }
}