use crate::{Parser, ParserDiagnostic};
use pxp_ast::utils::CommaSeparated;
use pxp_ast::Ending;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn skip_ending(&mut self) -> Ending {
        if self.current_kind() == TokenKind::CloseTag {
            Ending::CloseTag(self.next())
        } else if self.current_kind() == TokenKind::SemiColon {
            Ending::Semicolon(self.next())
        } else {
            let span = Span::flat(self.current_span().start);

            if self.is_eof() {
                self.diagnostic(ParserDiagnostic::UnexpectedEndOfFile, Severity::Error, span);
            } else {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::CloseTag, TokenKind::SemiColon],
                        found: self.current().to_owned(),
                    },
                    Severity::Error,
                    span,
                );
            }

            Ending::Missing(span)
        }
    }

    pub fn skip_semicolon(&mut self) -> Span {
        if self.current_kind() == TokenKind::SemiColon {
            self.next()
        } else {
            self.diagnostic(
                ParserDiagnostic::ExpectedToken {
                    expected: vec![TokenKind::SemiColon],
                    found: self.current().to_owned(),
                },
                Severity::Error,
                self.current_span(),
            );

            self.current_span()
        }
    }

    pub fn skip_left_brace(&mut self) -> Span {
        self.skip(TokenKind::LeftBrace)
    }

    pub fn skip_right_brace(&mut self) -> Span {
        self.skip(TokenKind::RightBrace)
    }

    pub fn skip_left_parenthesis(&mut self) -> Span {
        self.skip(TokenKind::LeftParen)
    }

    pub fn skip_right_parenthesis(&mut self) -> Span {
        self.skip(TokenKind::RightParen)
    }

    pub fn skip_left_bracket(&mut self) -> Span {
        self.skip(TokenKind::LeftBracket)
    }

    pub fn skip_right_bracket(&mut self) -> Span {
        self.skip(TokenKind::RightBracket)
    }

    pub fn skip_double_arrow(&mut self) -> Span {
        self.skip(TokenKind::DoubleArrow)
    }

    pub fn skip_double_colon(&mut self) -> Span {
        self.skip(TokenKind::DoubleColon)
    }

    pub fn skip_colon(&mut self) -> Span {
        self.skip(TokenKind::Colon)
    }

    pub fn expect(&mut self, kind: TokenKind) -> Span {
        if self.current_kind() != kind {
            self.diagnostic(
                ParserDiagnostic::ExpectedToken {
                    expected: vec![kind],
                    found: self.current().to_owned(),
                },
                Severity::Error,
                self.current_span(),
            );

            return Span::missing();
        }

        let span = self.current_span();

        self.next();

        span
    }

    pub fn skip(&mut self, kind: TokenKind) -> Span {
        while self.current_kind() != kind {
            if self.is_eof() {
                self.diagnostic(
                    ParserDiagnostic::UnexpectedEndOfFileExpected {
                        expected: vec![kind],
                    },
                    Severity::Error,
                    self.current_span(),
                );
                break;
            }

            self.diagnostic(
                ParserDiagnostic::ExpectedToken {
                    expected: vec![kind],
                    found: self.current().to_owned(),
                },
                Severity::Error,
                self.current_span(),
            );

            self.next();
        }

        let end = self.current_span();

        self.next();

        end
    }

    pub fn skip_any_of(&mut self, kinds: &[TokenKind]) -> Span {
        if kinds.contains(&self.current_kind()) {
            self.next()
        } else {
            self.diagnostic(
                ParserDiagnostic::ExpectedToken {
                    expected: kinds.to_vec(),
                    found: self.current().to_owned(),
                },
                Severity::Error,
                self.current_span(),
            );

            self.current_span()
        }
    }

    /// Parse an item that is surrounded by parentheses.
    ///
    /// This function will skip the left parenthesis, call the given function,
    /// and then skip the right parenthesis.
    pub fn parenthesized<T>(&mut self, mut func: impl FnMut(&mut Parser) -> T) -> (Span, T, Span) {
        let left_parenthesis = self.skip_left_parenthesis();
        let inner = func(self);
        let right_parenthesis = self.skip_right_parenthesis();

        (left_parenthesis, inner, right_parenthesis)
    }

    /// Parse an item that is surrounded by braces.
    ///
    /// This function will skip the left brace, call the given function,
    /// and then skip the right brace.
    pub fn braced<T>(&mut self, mut func: impl FnMut(&mut Parser) -> T) -> (Span, T, Span) {
        let left_brace = self.skip_left_brace();
        let inner = func(self);
        let right_brace = self.skip_right_brace();

        (left_brace, inner, right_brace)
    }

    pub fn semicolon_terminated<T>(&mut self, mut func: impl FnMut(&mut Parser) -> T) -> (Span, T) {
        let inner = func(self);
        let semicolon = self.skip_semicolon();
        (semicolon, inner)
    }

    /// Parse a comma-separated list of items, allowing a trailing comma.
    pub fn comma_separated<T>(
        &mut self,
        mut func: impl FnMut(&mut Parser) -> T,
        until: TokenKind,
    ) -> CommaSeparated<T> {
        let mut inner: Vec<T> = vec![];
        let mut commas: Vec<Span> = vec![];
        let mut current = self.current();

        while current.kind != until {
            inner.push(func(self));

            current = self.current();
            if current.kind != TokenKind::Comma {
                break;
            }

            commas.push(current.span);

            self.next();

            current = self.current();
        }

        CommaSeparated { inner, commas }
    }

    /// Parse a comma-separated list of items, not allowing trailing commas.
    pub fn comma_separated_no_trailing<T>(
        &mut self,
        mut func: impl FnMut(&mut Parser) -> T,
        until: TokenKind,
    ) -> CommaSeparated<T> {
        let mut inner: Vec<T> = vec![];
        let mut commas: Vec<Span> = vec![];

        while !self.is_eof() && self.current_kind() != until {
            inner.push(func(self));

            if self.current_kind() != TokenKind::Comma {
                break;
            }

            // If the next token is the until token, we don't want to consume the comma.
            // This ensures that trailing commas are not allowed.
            if self.peek_kind() == until {
                break;
            }

            commas.push(self.next());
        }

        CommaSeparated { inner, commas }
    }

    /// Parse a comma-separated list of items, requiring at least one item, and not allowing trailing commas.
    pub fn at_least_one_comma_separated_no_trailing<T>(
        &mut self,
        mut func: impl FnMut(&mut Parser) -> T,
    ) -> CommaSeparated<T> {
        let mut inner: Vec<T> = vec![];
        let mut commas: Vec<Span> = vec![];

        loop {
            inner.push(func(self));

            if self.current_kind() != TokenKind::Comma {
                break;
            }

            commas.push(self.next());
        }

        CommaSeparated { inner, commas }
    }
}
