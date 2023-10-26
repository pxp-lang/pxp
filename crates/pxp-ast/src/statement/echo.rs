use crate::Expression;

#[derive(Debug, Clone)]
pub struct EchoStatement {
    pub values: Vec<Expression>,
}