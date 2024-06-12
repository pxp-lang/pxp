use std::collections::HashMap;

use pxp_ast::{Name, Node, Statement};
use pxp_index::Index;
use pxp_symbol::Symbol;
use pxp_type::Type;
use pxp_visitor::{NodeVisitor, NodeVisitorResult};

#[derive(Debug, Clone)]
pub struct InferenceEngine;

impl InferenceEngine {
    pub fn new() -> Self {
        InferenceEngine
    }

    pub fn infer(&self, index: &Index, ast: &[Statement], target: &dyn Node) -> Type<Name> {
        let mut visitor = ContextTrackingNodeVisitor::new(index, target);

        visitor.visit_ast(ast);
        visitor.get_type()
    }
}

struct ContextTrackingNodeVisitor<'a> {
    index: &'a Index,
    target: &'a dyn Node,
    resolved: Type<Name>,
    contexts: Vec<Context>,
}

struct Context {
    variables: HashMap<Symbol, Type<Name>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            variables: HashMap::new(),
        }
    }

    pub fn stripped(&self) -> Self {
        Context {
            variables: HashMap::new(),
        }
    }

    pub fn cloned(&self) -> Self {
        Context {
            variables: self.variables.clone(),
        }
    }
}

impl<'a> ContextTrackingNodeVisitor<'a> {
    pub fn new(index: &'a Index, target: &'a dyn Node) -> Self {
        ContextTrackingNodeVisitor {
            index,
            target,
            resolved: Type::Mixed,
            contexts: Vec::from([Context::new()]),
        }
    }

    pub fn get_type(&self) -> Type<Name> {
        self.resolved.clone()
    }

    fn resolve_type(&self, node: &dyn Node) -> Type<Name> {
        Type::Mixed
    }

    fn process_node(&self, node: &dyn Node) {
        
    }

    fn push_context(&mut self) {
        self.contexts.push(Context::new());
    }

    fn pop_context(&mut self) {
        self.contexts.pop();
    }

    fn context_mut(&mut self) -> &mut Context {
        self.contexts.last_mut().unwrap()
    }

    fn context(&self) -> &Context {
        self.contexts.last().unwrap()
    }
}

impl<'a> NodeVisitor for ContextTrackingNodeVisitor<'a> {
    fn visit(&mut self, node: &dyn Node) -> NodeVisitorResult {
        if std::ptr::eq(node, self.target) {
            self.resolved = self.resolve_type(node);

            return NodeVisitorResult::Stop;
        }

        self.process_node(node);
               
        NodeVisitorResult::Continue
    }
}