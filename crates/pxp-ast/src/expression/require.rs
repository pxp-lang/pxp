use crate::Expression;

#[derive(Debug, Clone)]
pub struct RequireExpression {
    pub path: Box<Expression>,
}