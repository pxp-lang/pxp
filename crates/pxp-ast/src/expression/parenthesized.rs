use crate::Expression;

#[derive(Debug, Clone)]
pub struct ParenthesizedExpression {
    pub expression: Box<Expression>,
}