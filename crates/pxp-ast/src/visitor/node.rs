use crate::{Node, NodeKind, Statement};

#[derive(PartialEq, Eq)]
pub enum NodeVisitorEscapeHatch {
    SkipChildren,
    Stop,
    Continue,
}

pub trait NodeVisitor<'a> {
    fn enter(&mut self, _: Node<'a>, _: &mut Ancestors<'a>) -> NodeVisitorEscapeHatch {
        NodeVisitorEscapeHatch::Continue
    }

    fn leave(&mut self, _: Node<'a>, _: &mut Ancestors<'a>) -> NodeVisitorEscapeHatch {
        NodeVisitorEscapeHatch::Continue
    }

    fn visit(&mut self, node: Node<'a>, ancestors: &mut Ancestors<'a>) -> NodeVisitorEscapeHatch {
        let escape = self.enter(node.clone(), ancestors);

        ancestors.push(node.clone());

        if escape != NodeVisitorEscapeHatch::SkipChildren {
            for child in node.children() {
                self.visit(child, ancestors);
            }
        }

        if escape == NodeVisitorEscapeHatch::Stop {
            return NodeVisitorEscapeHatch::Stop;
        }

        ancestors.pop();

        self.leave(node, ancestors)
    }

    fn traverse(&mut self, ast: &'a [Statement]) {
        let mut ancestors = Ancestors::new();

        for statement in ast {
            let escape = self.visit(
                Node::new(
                    statement.id,
                    NodeKind::Statement(statement),
                    statement.span,
                ),
                &mut ancestors,
            );

            if escape == NodeVisitorEscapeHatch::Stop {
                break;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ancestors<'a> {
    ancestors: Vec<Node<'a>>,
}

impl<'a> Ancestors<'a> {
    fn new() -> Self {
        Self {
            ancestors: Vec::new(),
        }
    }

    fn push(&mut self, node: Node<'a>) {
        self.ancestors.push(node);
    }

    fn pop(&mut self) {
        self.ancestors.pop();
    }

    pub fn last(&self) -> Option<Node<'a>> {
        self.ancestors.last().cloned()
    }

    pub fn find(&self, cb: impl Fn(&Node<'a>) -> bool) -> Option<Node<'a>> {
        self.ancestors.iter().rev().find(|node| cb(node)).cloned()
    }
}