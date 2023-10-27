use crate::{Expression, SimpleIdentifier};

#[derive(Debug, Clone)]
pub struct ConstantFetchExpression {
    pub target: Box<Expression>,
    pub constant: SimpleIdentifier,
}