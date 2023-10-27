use crate::Expression;

#[derive(Debug, Clone)]
pub struct IncludeOnceExpression {
    pub path: Box<Expression>,
}