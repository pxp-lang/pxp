use crate::Argument;

#[derive(Debug, Clone)]
pub struct EvalExpression {
    pub argument: Box<Argument>,
}