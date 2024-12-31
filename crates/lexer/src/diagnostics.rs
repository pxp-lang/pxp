use std::fmt::Display;

use pxp_diagnostics::DiagnosticKind;

#[derive(Debug, Clone)]
pub enum LexerDiagnostic {
    UnexpectedEndOfFile,
    UnexpectedCharacter(u8),
    InvalidHaltCompiler,
    InvalidUnicodeEscapeSequence,
    InvalidOctalSequence,
}

impl DiagnosticKind for LexerDiagnostic {
    fn code(&self) -> &str {
        match self {
            Self::UnexpectedEndOfFile => "L001",
            Self::UnexpectedCharacter(_) => "L002",
            Self::InvalidHaltCompiler => "L003",
            Self::InvalidUnicodeEscapeSequence => "L004",
            Self::InvalidOctalSequence => "L005",
        }
    }

    fn identifier(&self) -> &str {
        match self {
            Self::UnexpectedEndOfFile => "lexer.unexpected-end-of-file",
            Self::UnexpectedCharacter(_) => "lexer.unexpected-character",
            Self::InvalidHaltCompiler => "lexer.invalid-halt-compiler",
            Self::InvalidUnicodeEscapeSequence => "lexer.invalid-unicode-escape-sequence",
            Self::InvalidOctalSequence => "lexer.invalid-octal-escape-sequence",
        }
    }
}

impl Display for LexerDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEndOfFile => write!(f, "unexpected end of file"),
            Self::UnexpectedCharacter(c) => write!(f, "unexpected character '{}'", *c as char),
            Self::InvalidHaltCompiler => write!(f, "invalid halt compiler directive"),
            Self::InvalidUnicodeEscapeSequence => write!(f, "invalid unicode escape sequence"),
            Self::InvalidOctalSequence => write!(f, "invalid octal escape sequence"),
        }
    }
}
