use pxp_parser::{traverser::Visitor, node::Node, parser::ast::variables::{Variable, SimpleVariable}, downcast::downcast};

#[derive(Default)]
pub struct VariableVisitor {
    variables: Vec<SimpleVariable>,
}

impl VariableVisitor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn find(&mut self, node: &mut dyn Node) -> Vec<SimpleVariable> {
        self.visit_node(node).unwrap();
        self.variables.clone()
    }
}

impl Visitor<()> for VariableVisitor {
    fn visit(&mut self, node: &mut dyn Node) -> Result<(), ()> {
        if let Some(Variable::SimpleVariable(variable)) = downcast::<Variable>(node) {
            self.variables.push(variable.clone());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use pxp_parser::{parse, parser::ast::variables::SimpleVariable, lexer::token::Span};
    use super::VariableVisitor;

    #[test]
    fn it_can_find_variables() {
        let mut ast = parse("<?php $var;").unwrap();
        let variables = VariableVisitor::new().find(&mut ast);

        assert_eq!(variables, vec![
            SimpleVariable {
                span: Span { line: 1, column: 7, position: 6 },
                name: "$var".into()
            }
        ]);
    }
}