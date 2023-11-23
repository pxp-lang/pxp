use std::fmt::Display;

use pxp_token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum DiagnosticKind {
    UnexpectedToken { token: Token },
    ExpectedToken { expected: Vec<TokenKind>, found: Token },
    UnexpectedEndOfFile,
}

impl Display for DiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticKind::UnexpectedToken { token } => write!(f, "unexpected token: {:?}", token),
            DiagnosticKind::ExpectedToken { expected, found } => if expected.len() == 1 {
                write!(f, "unexpected token {:?}, expected {:?}", found, expected)
            } else {
                write!(f, "unexpected token {:?}, expected one of {:?}", found, expected.iter().map(|kind| format!("{:?}", kind)).collect::<Vec<_>>().join(", "))
            },
            DiagnosticKind::UnexpectedEndOfFile => write!(f, "unexpected end of file"),
        }
    }
}