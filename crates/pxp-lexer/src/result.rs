pub type LexerResult<T> = Result<T, LexerError>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Hash)]
pub enum LexerError {
    UnpredictableState(usize),
    UnexpectedEndOfFile(usize),
    InvalidHaltCompiler(usize),
}