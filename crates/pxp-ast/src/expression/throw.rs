use crate::Expression;

#[derive(Debug, Clone)]
pub struct ThrowExpression {
    pub value: Box<Expression>,
}