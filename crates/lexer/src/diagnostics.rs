use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum LexerDiagnostic {
    UnexpectedEndOfFile,
    UnexpectedCharacter(u8),
    InvalidHaltCompiler,
    InvalidUnicodeEscapeSequence,
    InvalidOctalSequence,
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
