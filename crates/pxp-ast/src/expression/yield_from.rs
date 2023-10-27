use crate::Expression;

#[derive(Debug, Clone)]
pub struct YieldFromExpression {
    pub value: Box<Expression>,
}