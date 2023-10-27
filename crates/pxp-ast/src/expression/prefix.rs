use crate::Expression;

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub kind: PrefixOperator,
    pub rigvhht: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum PrefixOperator {
    Increment,
    Decrement,
    Negate,
    Positive,
    Not,
    BitwiseNot,
}