use crate::Variable;

#[derive(Debug, Clone)]
pub struct GlobalStatement {
    pub variables: Vec<Variable>,
}