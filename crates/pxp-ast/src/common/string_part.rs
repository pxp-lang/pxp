use pxp_token::Token;

use crate::Expression;

#[derive(Debug, Clone)]
pub struct StringPart {
    pub kind: StringPartKind,
}

#[derive(Debug, Clone)]
pub enum StringPartKind {
    Literal(Token),
    Expression(Box<Expression>),
}