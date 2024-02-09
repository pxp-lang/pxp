use pxp_span::{Position, Span};

use crate::token::Token;

#[derive(Debug)]
pub struct State<'a> {
    input: &'a [u8],
    start: Position,
    position: Position,
    tokens: Vec<Token>,
}

impl<'a> State<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            tokens: Vec::with_capacity(8),
            start: Position::new(0, 1, 0),
            position: Position::new(0, 1, 0),
        }
    }

    pub fn current(&self) -> u8 {
        if self.is_eof() {
            return self.input[self.position.offset - 1];
        }

        self.input[self.position.offset]
    }

    pub fn peek(&self) -> u8 {
        if self.is_eof() {
            return self.input[self.position.offset - 1];
        }

        self.input[self.position.offset + 1]
    }

    pub fn skip(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }

    pub fn next(&mut self) {
        if self.current() == b'\n' {
            self.position.line += 1;
            self.position.column = 0;
        } else {
            self.position.column += 1;
        }

        self.position.offset += 1;
    }

    pub fn skip_horizontal_whitespace(&mut self) {
        while self.current() == b'\x09' || self.current() == b'\x20' {
            self.next();
        }
    }

    pub fn span(&self) -> Span {
        Span::new(self.start, self.position)
    }

    pub fn start_token(&mut self) {
        self.start = self.position;
    }

    pub fn range(&self, start: usize, end: usize) -> &'a [u8] {
        &self.input[start..end]
    }

    pub fn peek_n(&self, n: usize) -> &'a [u8] {
        if self.position.offset + n >= self.input.len() {
            return &self.input[self.position.offset..];
        }

        &self.input[self.position.offset..(self.position.offset + n)]
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn get_tokens(self) -> Vec<Token> {
        self.tokens
    }

    pub fn is_eof(&self) -> bool {
        self.position.offset >= self.input.len()
    }
}
