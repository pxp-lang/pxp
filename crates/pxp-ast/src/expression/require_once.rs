use crate::Expression;

#[derive(Debug, Clone)]
pub struct RequireOnceExpression {
    pub path: Box<Expression>,
}