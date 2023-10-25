pub type LexerResult<T> = Result<T, LexerError>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Hash)]
pub enum LexerError {
    UnpredictableState(usize),
    UnexpectedEndOfFile(usize),
    InvalidHaltCompiler(usize),
    InvalidUnicodeEscape(usize),
    InvalidOctalEscape(usize),
    UnexpectedCharacter(u8, usize),
    UnrecognisedToken(u8, usize),
}