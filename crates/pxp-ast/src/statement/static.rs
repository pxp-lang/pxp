use crate::{Expression, SimpleVariable};

#[derive(Debug, Clone)]
pub struct StaticStatement {
    pub variables: Vec<StaticVariable>,
}

#[derive(Debug, Clone)]
pub struct StaticVariable {
    pub variable: SimpleVariable,
    pub value: Option<Expression>,
}