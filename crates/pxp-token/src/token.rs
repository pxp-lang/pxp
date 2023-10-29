use std::fmt::{Display, Debug};

use pxp_bytestring::ByteString;
use pxp_span::{Span, HasSpan};

use crate::TokenKind;

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub literal: ByteString,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, literal: ByteString) -> Self {
        Self { kind, span, literal }
    }

    pub fn missing(span: Span) -> Self {
        Self {
            kind: TokenKind::Missing,
            span,
            literal: ByteString::default(),
        }
    }

    #[inline(always)]
    pub const fn kind(&self) -> &TokenKind {
        &self.kind
    }

    #[inline(always)]
    pub const fn literal(&self) -> &ByteString {
        &self.literal
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({:?})", self.literal, self.kind)
    }
}

impl HasSpan for Token {
    fn span(&self) -> Span {
        self.span
    }
}