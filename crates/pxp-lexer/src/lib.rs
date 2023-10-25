mod lexer;
mod state;
mod result;
mod macros;
#[cfg(test)]
mod tests;

pub use lexer::Lexer;
pub use result::{LexerResult, LexerError};