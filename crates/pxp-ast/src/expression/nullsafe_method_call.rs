use crate::{Expression, ArgumentList};

#[derive(Debug, Clone)]
pub struct NullsafeMethodCallExpression {
    pub target: Box<Expression>,
    pub method: Box<Expression>,
    pub arguments: ArgumentList,
}