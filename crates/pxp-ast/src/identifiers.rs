use crate::Expression;
use pxp_span::Span;
use pxp_token::Token;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Identifier {
    SimpleIdentifier(SimpleIdentifier),
    DynamicIdentifier(DynamicIdentifier),
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct SimpleIdentifier {
    pub token: Token,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct DynamicIdentifier {
    pub span: Span,
    pub expr: Box<Expression>,
}
