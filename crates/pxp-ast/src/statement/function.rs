use crate::{SimpleIdentifier, Block, SimpleVariable, Expression};

#[derive(Debug, Clone)]
pub struct FunctionStatement {
    // FIXME: Add in attributes here.
    pub name: SimpleIdentifier,
    pub parameters: Vec<FunctionParameter>,
    // FIXME: Add in return type here.
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct FunctionParameter {
    pub name: SimpleVariable,
    // FIXME: Add in parameter type here.
    pub variadic: bool,
    pub by_reference: bool,
    pub default: Option<Expression>,
}