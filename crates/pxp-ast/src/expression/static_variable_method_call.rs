use crate::{Expression, Variable, ArgumentList};

#[derive(Debug, Clone)]
pub struct StaticVariableMethodCallExpression {
    pub target: Box<Expression>,
    pub method: Box<Variable>,
    pub arguments: ArgumentList,
}