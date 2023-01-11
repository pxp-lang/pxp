use pxp_parser::{
    node::{downcast, Node},
    parser::ast::variables::{SimpleVariable, Variable},
    traverser::Visitor,
};

#[derive(Default, Debug)]
pub struct VariableFinderVisitor {
    variables: Vec<SimpleVariable>,
    include_this: bool,
}

impl VariableFinderVisitor {
    pub fn new(include_this: bool) -> Self {
        Self {
            variables: Vec::new(),
            include_this,
        }
    }

    pub fn find(node: &dyn Node, include_this: bool) -> Vec<SimpleVariable> {
        let mut finder = Self::new(include_this);
        finder.visit_node(node).unwrap();
        finder.variables.clone()
    }
}

impl Visitor<()> for VariableFinderVisitor {
    fn visit(&mut self, node: &dyn Node) -> Result<(), ()> {
        if let Some(variable) = downcast::<Variable>(node) {
            match variable {
                Variable::SimpleVariable(variable) => match variable.name.as_slice() {
                    b"$this" if !self.include_this => {}
                    _ => self.variables.push(variable.clone()),
                },
                Variable::VariableVariable(_) => {}
                Variable::BracedVariableVariable(_) => {}
            }
        }

        Ok(())
    }
}
