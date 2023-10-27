use crate::Expression;

#[derive(Debug, Clone)]
pub struct ArrayIndexExpression {
    pub array: Box<Expression>,
    pub index: Option<Box<Expression>>,
}