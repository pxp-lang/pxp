use crate::Argument;

#[derive(Debug, Clone)]
pub struct ExitExpression {
    pub argument: Option<Box<Argument>>,
}