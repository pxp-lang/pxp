use pxp_bytestring::{ByteString, ByteStr};
use pxp_symbol::SymbolTable;

use crate::token::{Token, TokenKind};

pub struct State<'a> {
    tokens: &'a [Token],
    symbol_table: &'a SymbolTable,
    position: usize,
}

impl<'a> State<'a> {
    pub fn new(tokens: &'a [Token], symbol_table: &'a SymbolTable) -> Self {
        Self { tokens, symbol_table, position: 0 }
    }

    pub fn get_current_symbol(&self) -> Option<ByteStr> {
        let token = self.current();

        match token.kind.get_symbol() {
            Some(symbol) => self.symbol_table.resolve(symbol),
            None => None,
        }
    }

    pub fn current(&self) -> &'a Token {
        &self.tokens[self.position]
    }

    pub fn peek(&self) -> &'a Token {
        &self.tokens[self.position + 1]
    }

    pub fn is_eof(&self) -> bool {
        self.position >= self.tokens.len()
    }

    pub fn next(&mut self) {
        self.position += 1;
    }
}
