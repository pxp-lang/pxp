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
    fn get_code(&self) -> String {
        String::from(match self {
            Self::UnexpectedEndOfFile => "L001",
            Self::UnexpectedCharacter(_) => "L002",
            Self::InvalidHaltCompiler => "L003",
            Self::InvalidUnicodeEscapeSequence => "L004",
            Self::InvalidOctalSequence => "L005",
        })
    }

    fn get_identifier(&self) -> String {
        String::from(match self {
            Self::UnexpectedEndOfFile => "lexer.unexpected-end-of-file",
            Self::UnexpectedCharacter(_) => "lexer.unexpected-character",
            Self::InvalidHaltCompiler => "lexer.invalid-halt-compiler",
            Self::InvalidUnicodeEscapeSequence => "lexer.invalid-unicode-escape-sequence",
            Self::InvalidOctalSequence => "lexer.invalid-octal-escape-sequence",
        })
    }

    fn get_message(&self) -> String {
        String::from(match self {
            Self::UnexpectedEndOfFile => "unexpected end of file",
            Self::UnexpectedCharacter(_) => "unexpected character",
            Self::InvalidHaltCompiler => "invalid halt compiler directive",
            Self::InvalidUnicodeEscapeSequence => "invalid unicode escape sequence",
            Self::InvalidOctalSequence => "invalid octal escape sequence",
        })
    }
}
