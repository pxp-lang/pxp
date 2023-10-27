use crate::Expression;

#[derive(Debug, Clone)]
pub struct ShortTernaryExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}