use crate::Token;

#[derive(Debug)]
pub struct TokenStream<'a> {
    tokens: &'a [Token],
    index: usize,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            index: 0,
        }
    }

    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn is_eof(&self) -> bool {
        self.index >= self.tokens.len()
    }
}