use crate::Expression;

#[derive(Debug, Clone)]
pub struct PrintExpression {
    pub value: Box<Expression>,
}