use crate::{FunctionParameter, Expression};

#[derive(Debug, Clone)]
pub struct ArrowFunctionExpression {
    // FIXME: Add comments here.
    pub is_static: bool,
    pub by_reference: bool,
    // FIXME: Add attributes here.
    pub parameters: Vec<FunctionParameter>,
    // FIXME: Add return type here.
    pub body: Box<Expression>,
}