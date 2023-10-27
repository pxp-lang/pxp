use crate::Expression;

#[derive(Debug, Clone)]
pub struct ErrorSuppressExpression {
    pub expression: Box<Expression>,
}