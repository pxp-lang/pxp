use crate::Expression;

#[derive(Debug, Clone)]
pub struct FunctionClosureCreationExpression {
    pub target: Box<Expression>,
}