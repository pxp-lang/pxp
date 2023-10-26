use crate::{SimpleIdentifier, Expression};

#[derive(Debug, Clone)]
pub struct ConstantStatement {
    pub constants: Vec<Constant>,
}

#[derive(Debug, Clone)]
pub struct Constant {
    pub name: SimpleIdentifier,
    pub value: Expression,
}