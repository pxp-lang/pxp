use pxp_type::Type;

use crate::{SimpleIdentifier, Block, SimpleVariable, Expression, AttributeGroup};

#[derive(Debug, Clone)]
pub struct FunctionStatement {
    pub attributes: Vec<AttributeGroup>,
    pub name: SimpleIdentifier,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct FunctionParameter {
    pub name: SimpleVariable,
    pub r#type: Option<Type>,
    pub variadic: bool,
    pub by_reference: bool,
    pub default: Option<Expression>,
}