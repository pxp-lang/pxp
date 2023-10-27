use pxp_type::Type;

use crate::{FunctionParameter, Expression};

#[derive(Debug, Clone)]
pub struct ArrowFunctionExpression {
    // FIXME: Add comments here.
    pub is_static: bool,
    pub by_reference: bool,
    // FIXME: Add attributes here.
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<Type>,
    pub body: Box<Expression>,
}