use crate::Argument;

#[derive(Debug, Clone)]
pub struct DieExpression {
    pub argument: Option<Box<Argument>>,
}