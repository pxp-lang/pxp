use crate::{Expression, Identifier};

#[derive(Debug, Clone)]
pub struct StaticMethodClosureCreationExpression {
    pub target: Box<Expression>,
    pub method: Identifier,
}