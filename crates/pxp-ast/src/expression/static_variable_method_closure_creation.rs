use crate::{Expression, Variable};

#[derive(Debug, Clone)]
pub struct StaticVariableMethodClosureCreationExpression {
    pub target: Box<Expression>,
    pub method: Box<Variable>,
}