use parse::top_level_statement;
use pxp_ast::Program;
use pxp_token::Token;
use result::ParseResult;
use state::ParserState;
use token_stream::TokenStream;

mod token_stream;
mod state;
mod result;
mod parse;
mod macros;

pub fn construct(tokens: &[Token]) -> ParseResult {
    let mut stream = TokenStream::new(tokens);
    let mut state = ParserState::new(&mut stream);
    let mut program = Program::new();

    while !state.stream.is_eof() {
        program.push(top_level_statement(&mut state));
    }

    ParseResult { program, errors: state.errors }
}