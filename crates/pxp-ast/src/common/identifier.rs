use pxp_span::Span;
use pxp_token::{Token, TokenKind};

use crate::Expression;

#[derive(Debug, Clone)]
pub struct Identifier {
    pub kind: IdentifierKind,
    pub span: Span,
}

impl Identifier {
    pub fn simple(value: Token, span: Span) -> Self {
        Self {
            kind: IdentifierKind::Simple(SimpleIdentifier { value, span }),
            span
        }
    }

    pub fn dynamic(expression: Expression, span: Span) -> Self {
        Self {
            kind: IdentifierKind::Dynamic(DynamicIdentifier { expression: Box::new(expression), span }),
            span
        }
    }
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

    pub fn is_missing(&self) -> bool {
        self.value.kind == TokenKind::Missing
    }
}

#[derive(Debug, Clone)]
pub struct DynamicIdentifier {
    pub expression: Box<Expression>,
    pub span: Span,
}