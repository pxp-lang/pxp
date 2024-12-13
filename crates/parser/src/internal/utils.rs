use crate::state::State;
use crate::{Parser, ParserDiagnostic};
use pxp_ast::utils::CommaSeparated;
use pxp_ast::Ending;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn skip_ending(&mut self) -> Ending {
        let current = self.current();
        let previous = state.previous();

        if current.kind == TokenKind::CloseTag {
            self.next();

            Ending::CloseTag(current.span)
        } else if current.kind == TokenKind::SemiColon {
            self.next();

            Ending::Semicolon(current.span)
        } else {
            let span = Span::flat(previous.span.end);

            if state.is_eof() {
                self.diagnostic(ParserDiagnostic::UnexpectedEndOfFile, Severity::Error, span);
            } else {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::CloseTag, TokenKind::SemiColon],
                        found: current.clone(),
                    },
                    Severity::Error,
                    span,
                );
            }

            Ending::Missing(span)
        }
    }

    pub fn skip_semicolon(&mut self) -> Span {
        let current = self.current();

        if current.kind == TokenKind::SemiColon {
            self.next();

            current.span
        } else {
            self.diagnostic(
                ParserDiagnostic::ExpectedToken {
                    expected: vec![TokenKind::SemiColon],
                    found: current.clone(),
                },
                Severity::Error,
                current.span,
            );

            current.span
        }
    }

    pub fn skip_left_brace(&mut self) -> Span {
        skip(state, TokenKind::LeftBrace)
    }

    pub fn skip_right_brace(&mut self) -> Span {
        skip(state, TokenKind::RightBrace)
    }

    pub fn skip_left_parenthesis(&mut self) -> Span {
        skip(state, TokenKind::LeftParen)
    }

    pub fn skip_right_parenthesis(&mut self) -> Span {
        skip(state, TokenKind::RightParen)
    }

    pub fn skip_left_bracket(&mut self) -> Span {
        skip(state, TokenKind::LeftBracket)
    }

    pub fn skip_right_bracket(&mut self) -> Span {
        skip(state, TokenKind::RightBracket)
    }

    pub fn skip_double_arrow(&mut self) -> Span {
        skip(state, TokenKind::DoubleArrow)
    }

    pub fn skip_double_colon(&mut self) -> Span {
        skip(state, TokenKind::DoubleColon)
    }

    pub fn skip_colon(&mut self) -> Span {
        skip(state, TokenKind::Colon)
    }

    pub fn skip(&mut self, kind: TokenKind) -> Span {
        while self.current().kind != kind {
            let current = self.current();

            if state.is_eof() {
                self.diagnostic(
                    ParserDiagnostic::UnexpectedEndOfFileExpected {
                        expected: vec![kind],
                    },
                    Severity::Error,
                    current.span,
                );
                break;
            }

            self.next();

            self.diagnostic(
                ParserDiagnostic::ExpectedToken {
                    expected: vec![kind],
                    found: current.clone(),
                },
                Severity::Error,
                current.span,
            );
        }

        let end = self.current().span;

        self.next();

        end
    }

    pub fn skip_any_of(&mut self, kinds: &[TokenKind]) -> Span {
        let current = self.current();

        if kinds.contains(&current.kind) {
            let end = current.span;

            self.next();

            end
        } else {
            self.diagnostic(
                ParserDiagnostic::ExpectedToken {
                    expected: kinds.to_vec(),
                    found: current.clone(),
                },
                Severity::Error,
                current.span,
            );

            current.span
        }
    }

    /// Parse an item that is surrounded by parentheses.
    ///
    /// This function will skip the left parenthesis, call the given function,
    /// and then skip the right parenthesis.
    pub fn parenthesized<T>(&mut self, func: &(dyn Fn(&mut State) -> T)) -> (Span, T, Span) {
        let left_parenthesis = skip_left_parenthesis();
        let inner = func();
        let right_parenthesis = skip_right_parenthesis();

        (left_parenthesis, inner, right_parenthesis)
    }

    /// Parse an item that is surrounded by braces.
    ///
    /// This function will skip the left brace, call the given function,
    /// and then skip the right brace.
    pub fn braced<T>(&mut self, func: &(dyn Fn(&mut State) -> T)) -> (Span, T, Span) {
        let left_brace = skip_left_brace();
        let inner = func();
        let right_brace = skip_right_brace();

        (left_brace, inner, right_brace)
    }

    pub fn semicolon_terminated<T>(&mut self, func: &(dyn Fn(&mut State) -> T)) -> (Span, T) {
        let inner = func();
        let semicolon = skip_semicolon();
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
        func: &(dyn Fn(&mut State) -> T),
        until: TokenKind,
    ) -> CommaSeparated<T> {
        let mut inner: Vec<T> = vec![];
        let mut commas: Vec<Span> = vec![];
        let mut current = self.current();

        while current.kind != until {
            inner.push(func());

            current = self.current();
            if current.kind != TokenKind::Comma {
                break;
            }

            // If the next token is the until token, we don't want to consume the comma.
            // This ensures that trailing commas are not allowed.
            if state.peek().kind == until {
                break;
            }

            commas.push(current.span);

            self.next();

            current = self.current();
        }

        CommaSeparated { inner, commas }
    }

    /// Parse a comma-separated list of items, requiring at least one item, and not allowing trailing commas.
    pub fn at_least_one_comma_separated_no_trailing<T>(
        &mut self,
        func: &(dyn Fn(&mut State) -> T),
    ) -> CommaSeparated<T> {
        let mut inner: Vec<T> = vec![];
        let mut commas: Vec<Span> = vec![];

        loop {
            inner.push(func());

            let current = self.current();
            if current.kind != TokenKind::Comma {
                break;
            }

            commas.push(current.span);

            self.next();
        }

        CommaSeparated { inner, commas }
    }
}
