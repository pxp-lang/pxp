use crate::Expression;

#[derive(Debug, Clone)]
pub struct YieldExpression {
    pub key: Option<Box<Expression>>,
    pub value: Option<Box<Expression>>,
}