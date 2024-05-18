use std::fmt::Display;

use pxp_span::Span;

pub type SyntaxResult<T> = Result<T, SyntaxError>;

#[derive(Debug, Eq, PartialEq)]
pub enum SyntaxError {
    UnexpectedEndOfFile(Span),
    UnexpectedError(Span),
    UnexpectedCharacter(u8, Span),
    InvalidHaltCompiler(Span),
    InvalidOctalEscape(Span),
    InvalidOctalLiteral(Span),
    InvalidUnicodeEscape(Span),
    UnpredictableState(Span),
    InvalidDocIndentation(Span),
    InvalidDocBodyIndentationLevel(usize, Span),
    UnrecognisedToken(u8, Span),
}

impl SyntaxError {
    pub fn span(&self) -> Span {
        match self {
            Self::UnexpectedEndOfFile(span) => *span,
            Self::UnexpectedError(span) => *span,
            Self::UnexpectedCharacter(_, span) => *span,
            Self::InvalidHaltCompiler(span) => *span,
            Self::InvalidOctalEscape(span) => *span,
            Self::InvalidOctalLiteral(span) => *span,
            Self::InvalidUnicodeEscape(span) => *span,
            Self::UnpredictableState(span) => *span,
            Self::InvalidDocIndentation(span) => *span,
            Self::InvalidDocBodyIndentationLevel(_, span) => *span,
            Self::UnrecognisedToken(_, span) => *span,
        }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEndOfFile(_) => write!(
                f,
                "Syntax Error: unexpected end of file",
            ),
            Self::UnexpectedError(_) => write!(
                f,
                "Syntax Error: unexpected error",
                
            ),
            Self::UnexpectedCharacter(char, _) => write!(
                f,
                "Syntax Error: unexpected character `{:?}`",
                *char as char, 
            ),
            Self::InvalidHaltCompiler(_) => write!(
                f,
                "Syntax Error: invalid halt compiler",
            ),
            Self::InvalidOctalEscape(_) => write!(
                f,
                "Syntax Error: invalid octal escape",
            ),
            Self::InvalidOctalLiteral(_) => write!(
                f,
                "Syntax Error: invalid octal literal",
            ),
            Self::InvalidUnicodeEscape(_) => write!(
                f,
                "Syntax Error: invalid unicode escape",
            ),
            Self::UnpredictableState(_) => write!(
                f,
                "Syntax Error: Reached an unpredictable state",
            ),
            Self::InvalidDocIndentation(_) => write!(
                f,
                "Syntax Error: Invalid indentation - cannot use tabs and spaces",
            ),
            Self::InvalidDocBodyIndentationLevel(expected, _) => write!(
                f,
                "Syntax Error: Invalid body indentation level - expecting an indentation level of at least {}",
                expected,
            ),
            Self::UnrecognisedToken(token, _) => write!(
                f,
                "Syntax Error: Unrecognised token {}",
                *token as char,
            )
        }
    }
}
