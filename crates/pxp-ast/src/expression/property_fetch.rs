use crate::Expression;

#[derive(Debug, Clone)]
pub struct PropertyFetchExpression {
    pub target: Box<Expression>,
    pub property: Box<Expression>,
}