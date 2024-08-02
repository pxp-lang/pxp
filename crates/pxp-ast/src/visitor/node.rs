use crate::{Node, NodeKind, Statement};

pub trait NodeVisitor {
    fn enter(&mut self, _: &Node) {}

    fn leave(&mut self, _: &Node) {}

    fn visit(&mut self, node: &Node) {
        self.enter(node);

        for child in node.children() {
            self.visit(&child);
        }

        self.leave(node);
    }

    fn traverse(&mut self, ast: &[Statement]) {
        for statement in ast {
            self.visit(&Node::new(
                statement.id,
                NodeKind::Statement(statement),
                statement.span,
            ));
        }
    }
}