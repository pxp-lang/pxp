use crate::Expression;

#[derive(Debug, Clone)]
pub struct IncludeExpression {
    pub path: Box<Expression>,
}