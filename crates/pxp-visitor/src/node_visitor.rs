use pxp_ast::Node;

pub trait NodeVisitor {
    fn visit_node(&mut self, node: &dyn Node) {
        self.visit(node);

        for child in node.children() {
            self.visit_node(child);
        }
    }

    fn visit(&mut self, node: &dyn Node);
}