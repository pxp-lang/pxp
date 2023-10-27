use pxp_type::Type;

use crate::{FunctionParameter, Expression, AttributeGroup};

#[derive(Debug, Clone)]
pub struct ArrowFunctionExpression {
    // FIXME: Add comments here.
    pub is_static: bool,
    pub by_reference: bool,
    pub attributes: Vec<AttributeGroup>,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<Type>,
    pub body: Box<Expression>,
}