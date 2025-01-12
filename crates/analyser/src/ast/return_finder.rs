use pxp_ast::{visitor::{Ancestors, NodeVisitor, NodeVisitorEscapeHatch, Visitor}, Node, ReturnStatement, Statement};

pub(super) struct ReturnFinder<'a> {
    returns: Vec<&'a ReturnStatement>,
}

impl<'a> ReturnFinder<'a> {
    fn new() -> Self {
        Self {
            returns: Vec::new(),
        }
    }

    pub(super) fn find(ast: &'a [Statement]) -> Vec<&'a ReturnStatement> {
        let mut finder = Self::new();
        finder.traverse(ast);
        finder.returns
    }
}

impl<'a> NodeVisitor<'a> for ReturnFinder<'a> {
    fn enter(&mut self, node: Node<'a>, _: &mut Ancestors<'a>) -> NodeVisitorEscapeHatch {
        if !node.is_return_statement() {
            return NodeVisitorEscapeHatch::Continue;
        }

        self.returns.push(node.as_return_statement().unwrap());

        NodeVisitorEscapeHatch::Continue
    }
}
