use crate::Expression;
use pxp_span::Span;
use pxp_token::Token;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Identifier {
    SimpleIdentifier(SimpleIdentifier),
    DynamicIdentifier(DynamicIdentifier),
}

impl Identifier {
    pub fn missing(token: Token) -> Self {
        Self::SimpleIdentifier(SimpleIdentifier { token })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct SimpleIdentifier {
    pub token: Token,
}

impl SimpleIdentifier {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct DynamicIdentifier {
    pub span: Span,
    pub expr: Box<Expression>,
}
