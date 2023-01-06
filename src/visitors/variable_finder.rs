use pxp_parser::{parser::ast::variables::{Variable, SimpleVariable}, traverser::Visitor, node::{Node, downcast}};

#[derive(Default, Debug)]
pub struct VariableFinderVisitor {
    pub variables: Vec<SimpleVariable>,
}

impl VariableFinderVisitor {
    pub fn variables(&self) -> &[SimpleVariable] {
        &self.variables.as_slice()
    }
}

#[derive(Debug)]
pub enum VariableFinderVisitorError {

}

impl Visitor<VariableFinderVisitorError> for VariableFinderVisitor {
    fn visit(&mut self, node: &dyn Node) -> Result<(), VariableFinderVisitorError> {
        if let Some(variable) = downcast::<Variable>(node) {
            match variable {
                Variable::SimpleVariable(variable) => {
                    match variable.name.as_slice() {
                        b"$this" => {},
                        _ => {
                            self.variables.push(variable.clone())
                        }
                    }
                },
                Variable::VariableVariable(_) => {},
                Variable::BracedVariableVariable(_) => {},
            }
        }

        Ok(())
    }
}