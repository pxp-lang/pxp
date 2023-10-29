use crate::{token_stream::TokenStream, result::ParseError};

#[derive(Debug)]
pub struct ParserState<'a> {
    pub stream: &'a mut TokenStream<'a>,
    pub errors: Vec<ParseError>,
}

impl<'a> ParserState<'a> {
    pub fn new(stream: &'a mut TokenStream<'a>) -> ParserState<'a> {
        ParserState {
            stream,
            errors: Vec::new(),
        }
    }
}