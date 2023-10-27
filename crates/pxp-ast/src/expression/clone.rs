use crate::Expression;

#[derive(Debug, Clone)]
pub struct CloneExpression {
    pub target: Box<Expression>,
}