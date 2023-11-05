use pxp_token::Token;

use crate::{Expression, Variable};

#[derive(Debug, Clone)]
pub struct StringPart {
    pub kind: StringPartKind,
}

impl StringPart {
    pub fn literal(token: Token) -> Self {
        Self { kind: StringPartKind::Literal(token) }
    }

    pub fn expression(expression: Expression) -> Self {
        Self { kind: StringPartKind::Expression(Box::new(expression)) }
    }
}

#[derive(Debug, Clone)]
pub enum StringPartKind {
    Literal(Token),
    Variable(Box<Variable>),
    Expression(Box<Expression>),
}