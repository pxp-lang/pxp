use pxp_span::Position;
use pxp_token::{Token, TokenKind};

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticKind {
    UnexpectedToken {
        token: Token,
    },
    ExpectedToken {
        expected: TokenKind,
        found: Token,
    },
    UnexpectedEndOfFile {
        position: Position,
    }
}