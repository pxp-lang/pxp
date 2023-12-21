use pxp_symbol::SymbolTable;

use crate::{token::{Token, TokenKind}, ast::Node};

use self::state::State;

mod state;

pub struct Parser;

impl Parser {
    pub const fn new() -> Self {
        Self
    }

    pub fn parse(&self, tokens: &[Token], symbol_table: &SymbolTable) -> ParseResult<Vec<Node>> {
        let mut state = State::new(tokens, symbol_table);
        let mut nodes = Vec::new();

        if state.current().kind != TokenKind::OpenPhpdoc {
            return Err(ParseError::MissingPhpdocOpen);
        }

        state.next();

        if let TokenKind::PhpdocEol(_) = state.current().kind {
            state.next();
        }

        while ! state.is_eof() {
            if state.current().kind == TokenKind::ClosePhpdoc {
                break;
            }

            let current = state.current();

            match current.kind {
                TokenKind::HorizontalWhitespace(_) => {
                    state.next();
                }
                _ => unimplemented!("{:?}", current.with_symbol_table(symbol_table))
            }
        }

        if state.current().kind != TokenKind::ClosePhpdoc {
            return Err(ParseError::MissingPhpdocClose);
        }

        state.next();

        Ok(nodes)
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    MissingPhpdocOpen,
    MissingPhpdocClose,
}
