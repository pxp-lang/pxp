use pxp_token::Token;

use crate::Expression;

#[derive(Debug, Clone)]
pub struct CastExpression {
    pub kind: CastKind,
    pub value: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum CastKind {
    Int(Token),
    Float(Token),
    String(Token),
    Array(Token),
    Object(Token),
    Bool(Token),
    Unset(Token),
    Binary(Token),
}