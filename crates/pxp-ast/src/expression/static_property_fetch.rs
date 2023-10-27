use crate::{Expression, Variable};

#[derive(Debug, Clone)]
pub struct StaticPropertyFetchExpression {
    pub target: Box<Expression>,
    pub property: Box<Variable>,
}