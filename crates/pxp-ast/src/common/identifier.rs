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
    pub span: Span,
}

impl SimpleIdentifier {
    pub fn missing(span: Span) -> Self {
        Self {
            value: Token::missing(span),
            span
        }
    }
}

#[derive(Debug, Clone)]
pub struct DynamicIdentifier {
    pub expression: Box<Expression>,
    pub span: Span,
}