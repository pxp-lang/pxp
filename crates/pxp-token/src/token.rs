use pxp_bytestring::ByteString;
use pxp_span::{Span, HasSpan};

use crate::TokenKind;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub literal: ByteString,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, literal: ByteString) -> Self {
        Self { kind, span, literal }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn literal(&self) -> &ByteString {
        &self.literal
    }
}

impl HasSpan for Token {
    fn span(&self) -> Span {
        self.span
    }
}