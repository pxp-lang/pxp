use crate::{Expression, Identifier, ArgumentList};

#[derive(Debug, Clone)]
pub struct StaticMethodCallExpression {
    pub target: Box<Expression>,
    pub method: Identifier,
    pub arguments: ArgumentList,
}