use pxp_type::Type;

use crate::{FunctionParameter, Block, SimpleVariable, AttributeGroup};

#[derive(Debug, Clone)]
pub struct ClosureExpression {
    // FIXME: Add comments here.
    pub attributes: Vec<AttributeGroup>,
    pub is_static: bool,
    pub by_reference: bool,
    pub parameters: Vec<FunctionParameter>,
    pub uses: Option<ClosureUse>,
    pub return_type: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct ClosureUse {
    pub variable: SimpleVariable,
    pub by_reference: bool,
}