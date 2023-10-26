use pxp_span::Span;
use pxp_token::Token;

use crate::Expression;

#[derive(Debug, Clone)]
pub struct Identifier {
    pub kind: IdentifierKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum IdentifierKind {
    Simple(SimpleIdentifier),
    Dynamic(DynamicIdentifier),
}

#[derive(Debug, Clone)]
pub struct SimpleIdentifier {
    pub value: Token,
}

#[derive(Debug, Clone)]
pub struct DynamicIdentifier {
    pub expression: Box<Expression>,
}