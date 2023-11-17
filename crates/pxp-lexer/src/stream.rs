use pxp_token::Token;
use pxp_token::TokenKind;

use pxp_ast::comments::Comment;
use pxp_ast::comments::CommentFormat;
use pxp_ast::comments::CommentGroup;

/// Token stream.
///
/// # Examples
///
/// ```rust
/// use pxp_lexer::stream::TokenStream;
/// use pxp_span::Span;
/// use pxp_token::Token;
/// use pxp_token::TokenKind;
///
/// let tokens = vec![
///     Token {
///         kind: TokenKind::SingleLineComment,
///         span: Span { start: 1, end: 1 },
///     },
///     Token {
///         kind: TokenKind::Readonly,
///         span: Span { start: 2, end: 1 },
///     },
///     Token {
///         kind: TokenKind::Class,
///         span: Span { start: 2, end: 0 },
///     },
///     Token {
///         kind: TokenKind::Enum,
///         span: Span { start: 2, end: 6 },
///     },
///     Token {
///         kind: TokenKind::LeftBrace,
///         span: Span { start: 2, end: 1 },
///     },
///     Token {
///         kind: TokenKind::SingleLineComment,
///         span: Span { start: 3, end: 1 },
///     },
///     Token {
///         kind: TokenKind::RightBrace,
///         span: Span { start: 4, end: 1 },
///     },
///     Token {
///         kind: TokenKind::Eof,
///         span: Span { start: 0, end: 0 },
///     },
/// ];
///
/// let mut stream = TokenStream::new(tokens);
///
/// assert!(matches!(stream.current().kind, TokenKind::Readonly));
/// assert!(matches!(stream.peek().kind, TokenKind::Class));
/// assert!(matches!(stream.lookahead(1).kind, TokenKind::Enum));
/// assert!(matches!(stream.lookahead(2).kind, TokenKind::LeftBrace));
/// assert!(matches!(stream.lookahead(3).kind, TokenKind::RightBrace));
/// assert!(matches!(stream.lookahead(4).kind, TokenKind::Eof));
/// assert!(matches!(stream.lookahead(5).kind, TokenKind::Eof));
///
/// stream.next();
///
/// assert!(matches!(stream.current().kind, TokenKind::Class));
///
/// stream.next();
/// stream.next();
/// stream.next();
///
/// assert!(matches!(stream.current().kind, TokenKind::RightBrace));
///
/// stream.next();
///
/// assert!(matches!(stream.current().kind, TokenKind::Eof));
/// assert!(stream.is_eof());
///
/// assert_eq!(
///     stream.comments(),
///     vec![
///         Token {
///             kind: TokenKind::SingleLineComment,
///             span: Span { start: 1, end: 1 }
///         },
///         Token {
///             kind: TokenKind::SingleLineComment,
///             span: Span { start: 3, end: 1 }
///         },
///     ]
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenStream<'a> {
    tokens: &'a [Token],
    length: usize,
    comments: Vec<&'a Token>,
    cursor: usize,
}

/// Token stream.
impl<'a> TokenStream<'a> {
    pub fn new(tokens: &'a [Token]) -> TokenStream {
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
                    | TokenKind::DocumentComment
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

    /// Get all comments.
    #[allow(dead_code)]
    pub fn comments(&mut self) -> CommentGroup {
        let mut comments = vec![];

        std::mem::swap(&mut self.comments, &mut comments);

        CommentGroup {
            comments: comments
                .iter()
                .map(|token| match token {
                    Token {
                        kind: TokenKind::SingleLineComment,
                        span,
                        value,
                    } => Comment {
                        span: *span,
                        format: CommentFormat::SingleLine,
                        content: value.clone(),
                    },
                    Token {
                        kind: TokenKind::MultiLineComment,
                        span,
                        value,
                    } => Comment {
                        span: *span,
                        format: CommentFormat::MultiLine,
                        content: value.clone(),
                    },
                    Token {
                        kind: TokenKind::HashMarkComment,
                        span,
                        value,
                    } => Comment {
                        span: *span,
                        format: CommentFormat::HashMark,
                        content: value.clone(),
                    },
                    Token {
                        kind: TokenKind::DocumentComment,
                        span,
                        value,
                    } => Comment {
                        span: *span,
                        format: CommentFormat::Document,
                        content: value.clone(),
                    },
                    _ => unreachable!(),
                })
                .collect(),
        }
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
                    | TokenKind::DocumentComment
            ) {
                break;
            }

            self.comments.push(current);
            self.cursor += 1;
        }
    }
}

impl<'a> Default for TokenStream<'a> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<'a> From<&'a Vec<Token>> for TokenStream<'a> {
    fn from(tokens: &'a Vec<Token>) -> Self {
        Self::new(tokens.as_slice())
    }
}
