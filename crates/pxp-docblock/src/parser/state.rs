use pxp_bytestring::{ByteString, ByteStr};
use pxp_symbol::SymbolTable;

use crate::token::{Token, TokenKind};

pub struct State<'a> {
    tokens: &'a [Token],
    pub symbol_table: &'a mut SymbolTable,
    position: usize,
}

impl<'a> State<'a> {
    pub fn new(tokens: &'a [Token], symbol_table: &'a mut SymbolTable) -> Self {
        Self { tokens, symbol_table, position: 0 }
    }

    pub fn get_current_symbol(&self) -> ByteStr {
        let token = self.current();

        self.symbol_table.resolve(token.symbol).unwrap()
    }

    pub fn skip_horizontal_whitespace(&mut self) {
        while self.current().kind == TokenKind::HorizontalWhitespace {
            self.next();
        }
    }

    pub fn current(&self) -> &'a Token {
        &self.tokens[self.position]
    }

    pub fn previous(&self) -> &'a Token {
        &self.tokens[self.position - 1]
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
