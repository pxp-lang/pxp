use std::fmt::Debug;

use crate::node::Node;

pub trait Visitor<E: Debug> {
    fn visit_node(&mut self, node: &mut dyn Node) -> Result<(), E> {
        self.visit(node)?;

        for child in node.children() {
            self.visit_node(child)?;
        }

        Ok(())
    }

    fn visit(&mut self, node: &mut dyn Node) -> Result<(), E>;
}
