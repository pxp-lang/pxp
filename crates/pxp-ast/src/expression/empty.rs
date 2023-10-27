use crate::Argument;

#[derive(Debug, Clone)]
pub struct EmptyExpression {
    pub argument: Box<Argument>,
}