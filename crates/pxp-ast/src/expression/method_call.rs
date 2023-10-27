use crate::{Expression, ArgumentList};

#[derive(Debug, Clone)]
pub struct MethodCallExpression {
    pub target: Box<Expression>,
    pub method: Box<Expression>,
    pub arguments: ArgumentList,
}