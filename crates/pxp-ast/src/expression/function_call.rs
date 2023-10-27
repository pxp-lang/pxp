use crate::{Expression, ArgumentList};

#[derive(Debug, Clone)]
pub struct FunctionCallExpression {
    pub target: Box<Expression>,
    pub arguments: ArgumentList,
}