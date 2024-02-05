use crate::Expression;
use pxp_span::Span;
use pxp_token::{Token, TokenKind};

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Identifier {
    SimpleIdentifier(SimpleIdentifier),
    DynamicIdentifier(DynamicIdentifier),
}

impl Identifier {
    pub fn missing(token: Token) -> Self {
        Self::SimpleIdentifier(SimpleIdentifier { token })
    }

    pub fn is_simple(&self) -> bool {
        match self {
            Self::SimpleIdentifier(..) => true,
            Self::DynamicIdentifier(..) => false,
        }
    }

    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::SimpleIdentifier(..) => false,
            Self::DynamicIdentifier(..) => true,
        }
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

    pub fn is_fully_qualified(&self) -> bool {
        matches!(self.token.kind, TokenKind::FullyQualifiedIdentifier)
    }

    pub fn is_qualified(&self) -> bool {
        matches!(self.token.kind, TokenKind::QualifiedIdentifier)
    }

    pub fn is_unqualified(&self) -> bool {
        matches!(self.token.kind, TokenKind::Identifier)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct DynamicIdentifier {
    pub span: Span,
    pub expr: Box<Expression>,
}
