use crate::SimpleVariable;

#[derive(Debug, Clone)]
pub struct ReferenceExpression {
    pub right: SimpleVariable,
}