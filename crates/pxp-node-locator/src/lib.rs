use pxp_ast::{Node, Statement};
use pxp_visitor::{NodeVisitor, NodeVisitorResult};

pub struct NodeLocator<'a> {
    ast: &'a [Statement],
    offset: usize,
    result: Option<&'a Statement>,
}

impl<'a> NodeLocator<'a> {
    pub fn locate(ast: &'a [Statement], offset: usize) -> Option<&'a Statement> {
        let mut locator = Self { ast, offset, result: None };
        
        locator.visit_ast(ast);
        locator.result
    }
}

impl<'a> NodeVisitor for NodeLocator<'a> {
    fn visit(&mut self, node: &dyn Node) -> NodeVisitorResult {
        NodeVisitorResult::Continue
    }
}