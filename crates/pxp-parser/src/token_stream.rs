use pxp_span::Span;
use pxp_token::Token;
use pxp_token::TokenKind;

#[derive(Debug, Clone)]
pub struct TokenStream<'a> {
    tokens: &'a [Token],
    length: usize,
    pub comments: Vec<&'a Token>,
    cursor: usize,
}

/// Token stream.
impl<'a> TokenStream<'a> {
    pub fn new(tokens: &'a [Token]) -> TokenStream<'a> {
        let length = tokens.len();

        let mut stream = TokenStream {
            tokens,
            length,
            comments: vec![],
            cursor: 0,
        };

        stream.collect_comments();

        stream
    }

    /// Move cursor to next token.
    ///
    /// Comments are collected.
    pub fn next(&mut self) {
        self.cursor += 1;
        self.collect_comments();
    }

    pub fn span(&self) -> Span {
        if self.cursor >= self.length {
            return self.previous().span;
        }

        self.current().span
    }

    /// Get current token.
    pub const fn current(&self) -> &'a Token {
        let position = if self.cursor >= self.length {
            self.length - 1
        } else {
            self.cursor
        };

        &self.tokens[position]
    }

    /// Get previous token.
    pub const fn previous(&self) -> &'a Token {
        let position = if self.cursor == 0 { 0 } else { self.cursor - 1 };
        let position = if position >= self.length {
            self.length - 1
        } else {
            position
        };

        &self.tokens[position]
    }

    /// Peek next token.
    ///
    /// All comments are skipped.
    pub const fn peek(&self) -> &'a Token {
        self.peek_nth(1)
    }

    /// Peek nth+1 token.
    ///
    /// All comments are skipped.
    pub const fn lookahead(&self, n: usize) -> &'a Token {
        self.peek_nth(n + 1)
    }

    /// Peek nth token.
    ///
    /// All comments are skipped.
    #[inline(always)]
    const fn peek_nth(&self, n: usize) -> &'a Token {
        let mut cursor = self.cursor + 1;
        let mut target = 1;
        loop {
            if cursor >= self.length {
                return &self.tokens[self.length - 1];
            }

            let current = &self.tokens[cursor];

            if matches!(
                current.kind,
                TokenKind::SingleLineComment
                    | TokenKind::MultiLineComment
                    | TokenKind::HashMarkComment
            ) {
                cursor += 1;
                continue;
            }

            if target == n {
                return current;
            }

            target += 1;
            cursor += 1;
        }
    }

    /// Check if current token is EOF.
    pub fn is_eof(&self) -> bool {
        if self.cursor >= self.length {
            return true;
        }

        self.tokens[self.cursor].kind == TokenKind::Eof
    }

    fn collect_comments(&mut self) {
        loop {
            if self.cursor >= self.length {
                break;
            }

            let current = &self.tokens[self.cursor];

            if !matches!(
                current.kind,
                TokenKind::SingleLineComment
                    | TokenKind::MultiLineComment
                    | TokenKind::HashMarkComment
            ) {
                break;
            }

            self.comments.push(current);
            self.cursor += 1;
        }
    }
}
