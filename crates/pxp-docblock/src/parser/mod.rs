use pxp_span::Span;
use pxp_symbol::SymbolTable;

use crate::{token::{Token, TokenKind}, ast::{Node, NodeKind, Text}};

use self::state::State;

mod state;

pub struct Parser;

impl Parser {
    pub const fn new() -> Self {
        Self
    }

    pub fn parse(&self, tokens: &[Token], symbol_table: &mut SymbolTable) -> ParseResult<Vec<Node>> {
        let mut state = State::new(tokens, symbol_table);
        let mut nodes = Vec::new();

        if state.current().kind != TokenKind::OpenPhpdoc {
            return Err(ParseError::MissingPhpdocOpen);
        }

        state.next();

        if let TokenKind::PhpdocEol = state.current().kind {
            state.next();
        }

        while state.current().kind != TokenKind::ClosePhpdoc {
            let current = state.current();

            match current.kind {
                TokenKind::PhpdocEol => {
                    state.next();
                },
                TokenKind::HorizontalWhitespace => {
                    state.next();
                }
                _ => {
                    let node = self.parse_text(&mut state)?;
                    nodes.push(node);
                }
            }
        }

        if state.current().kind != TokenKind::ClosePhpdoc {
            return Err(ParseError::MissingPhpdocClose);
        }

        state.next();

        Ok(nodes)
    }

    fn parse_text(&self, state: &mut State) -> ParseResult<Node> {
        let start_span = state.current().span;
        let mut symbols = Vec::new();

        loop {
            if state.is_eof() {
                break;
            }

            let current = state.current();

            match current.kind {
                TokenKind::PhpdocEol => break,
                TokenKind::ClosePhpdoc => break,
                _ => {
                    state.next();

                    symbols.push(current.symbol);
                }
            }
        }

        let end_span = state.previous().span;
        let span = Span::new(start_span.start, end_span.end);
        let symbol = state.symbol_table.coagulate(&symbols);

        Ok(Node::new(NodeKind::Text(Text::new(symbol)), span))
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    MissingPhpdocOpen,
    MissingPhpdocClose,
}
