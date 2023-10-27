use crate::Expression;

#[derive(Debug, Clone)]
pub struct MethodClosureCreationExpression {
    pub target: Box<Expression>,
    pub method: Box<Expression>,
}