use crate::Expression;

#[derive(Debug, Clone)]
pub struct PostfixExpression {
    pub kind: PostfixOperator,
    pub left: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum PostfixOperator {
    Increment,
    Decrement,
}