use crate::{Expression, ArgumentList};

#[derive(Debug, Clone)]
pub struct NewExpression {
    pub target: Box<Expression>,
    pub arguments: Option<ArgumentList>,
}