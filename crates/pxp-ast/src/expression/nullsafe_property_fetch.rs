use crate::Expression;

#[derive(Debug, Clone)]
pub struct NullsafePropertyFetchExpression {
    pub target: Box<Expression>,
    pub property: Box<Expression>,
}