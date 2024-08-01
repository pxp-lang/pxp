use crate::{Node, NodeKind, Statement};

pub trait NodeVisitor {
    fn enter(&mut self, node: &Node) {}

    fn leave(&mut self, node: &Node) {}

    fn visit(&mut self, node: &Node) {
        self.enter(node);

        // TODO: Traverse children of the node here.

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
