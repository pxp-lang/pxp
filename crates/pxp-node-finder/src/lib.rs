use pxp_ast::{Node, Statement, visitor::Visitor};
use pxp_span::ByteOffset;

pub struct NodeFinder;

impl NodeFinder {
    pub fn find_at_offset<'a>(offset: ByteOffset, ast: &'a Vec<Statement>) -> Option<Node<'a>> {
        let mut visitor = NodeFindingVisitor::new(offset);

        visitor.visit(ast);
        visitor.found
    }
}

struct NodeFindingVisitor<'a> {
    found: Option<Node<'a>>,
    offset: ByteOffset,
}

impl<'a> NodeFindingVisitor<'a> {
    pub fn new(offset: ByteOffset) -> Self {
        Self {
            found: None,
            offset,
        }
    }
}

impl<'a> Visitor for NodeFindingVisitor<'a> {
    
}