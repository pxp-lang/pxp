use pxp_span::Span;
use pxp_symbol::{SymbolTable, Symbol};

use crate::{token::{Token, TokenKind}, ast::{Node, NodeKind, Text, Tag, TagKind}};

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
                TokenKind::PhpdocTag => {
                    let node = self.parse_tag(&mut state)?;
                    nodes.push(node);
                },
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

    fn parse_tag(&self, state: &mut State) -> ParseResult<Node> {
        let start_span = state.current().span;
        let tag_token = state.current();
        state.next();
        let tag = state.symbol_table.resolve(tag_token.symbol).unwrap();

        Ok(match tag {
            _ => {
                let description = self.parse_optional_description(state)?;
                let end_span = state.previous().span;
                let span = Span::new(start_span.start, end_span.end);

                Node::new(NodeKind::Tag(Tag::new(TagKind::Generic { tag: tag_token.symbol, description }, span)), span)
            },
        })
    }

    fn parse_optional_description(&self, state: &mut State) -> ParseResult<Option<Symbol>> {
        if state.is_eof() {
            return Ok(None);
        }

        let current = state.current();

        match current.kind {
            TokenKind::PhpdocEol => {
                Ok(None)
            },
            _ => {
                Ok(Some(self.parse_text_symbol(state)?))
            }
        }
    }

    fn parse_text_symbol(&self, state: &mut State) -> ParseResult<Symbol> {
        let mut symbols = Vec::new();

        // We don't care about leading whitespace in the description.
        if state.current().kind == TokenKind::HorizontalWhitespace {
            state.next();
        }

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

        Ok(state.symbol_table.coagulate(&symbols))
    }

    fn parse_text(&self, state: &mut State) -> ParseResult<Node> {
        let start_span = state.current().span;
        let symbol = self.parse_text_symbol(state)?;
        let end_span = state.previous().span;
        let span = Span::new(start_span.start, end_span.end);

        Ok(Node::new(NodeKind::Text(Text::new(symbol)), span))
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    MissingPhpdocOpen,
    MissingPhpdocClose,
}
