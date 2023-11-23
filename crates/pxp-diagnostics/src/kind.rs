use std::fmt::Display;

use pxp_span::Position;
use pxp_token::{Token, TokenKind};

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticKind {
    UnexpectedToken { token: Token },
    ExpectedToken { expected: TokenKind, found: Token },
    UnexpectedEndOfFile,
}

impl Display for DiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticKind::UnexpectedToken { token } => write!(f, "unexpected token: {:?}", token),
            DiagnosticKind::ExpectedToken { expected, found } => write!(f, "unexpected token {:?}, expected {:?}", found, expected),
            DiagnosticKind::UnexpectedEndOfFile => write!(f, "unexpected end of file"),
        }
    }
}