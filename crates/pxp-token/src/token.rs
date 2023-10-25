use pxp_span::{Span, HasSpan};

use crate::TokenKind;

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}

impl HasSpan for Token {
    fn span(&self) -> Span {
        self.span
    }
}