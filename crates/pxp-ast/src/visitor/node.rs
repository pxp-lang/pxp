use crate::{Node, NodeKind, Statement};

#[derive(PartialEq, Eq)]
pub enum NodeVisitorEscapeHatch {
    SkipChildren,
    Stop,
    Continue,
}

pub trait NodeVisitor<'a> {
    fn enter(&mut self, _: Node<'a>) -> NodeVisitorEscapeHatch {
        NodeVisitorEscapeHatch::Continue
    }

    fn leave(&mut self, _: Node<'a>) -> NodeVisitorEscapeHatch {
        NodeVisitorEscapeHatch::Continue
    }

    fn visit(&mut self, node: Node<'a>) -> NodeVisitorEscapeHatch {
        let escape = self.enter(node.clone());

        if escape != NodeVisitorEscapeHatch::SkipChildren {
            for child in node.children() {
                self.visit(child);
            }
        }

        if escape == NodeVisitorEscapeHatch::Stop {
            return NodeVisitorEscapeHatch::Stop;
        }

        self.leave(node) 
    }

    fn traverse(&mut self, ast: &'a [Statement]) {
        for statement in ast {
            let escape = self.visit(Node::new(
                statement.id,
                NodeKind::Statement(statement),
                statement.span,
            ));

            if escape == NodeVisitorEscapeHatch::Stop {
                break;
            }
        }
    }
}
