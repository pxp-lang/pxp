mod diagnostics;
mod expressions;
mod internal;
mod macros;
mod state;

pub use diagnostics::ParserDiagnostic;
use pxp_ast::Statement;
use pxp_bytestring::{ByteStr, ByteString};
use pxp_diagnostics::{Diagnostic, Severity};
use pxp_lexer::Lexer;
use pxp_span::Span;
use pxp_token::{Token, TokenKind};
use state::State;

#[derive(Debug)]
pub struct ParseResult {
    pub ast: Vec<Statement>,
    pub diagnostics: Vec<Diagnostic<ParserDiagnostic>>,
}

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    state: State,
}

impl<'a> Parser<'a> {
    pub fn parse(lexer: Lexer<'a>) -> ParseResult {
        let mut parser = Parser::new(lexer);
        let mut ast = Vec::new();

        while !parser.is_eof() {
            ast.push(parser.parse_top_level_statement());
        }

        ParseResult {
            ast,
            diagnostics: parser.state.diagnostics.clone(),
        }
    }

    fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            state: State::new(),
        }
    }

    fn next(&mut self) -> Span {
        let span = self.current_span();

        self.lexer.next();

        span
    }

    fn is_eof(&self) -> bool {
        self.current_kind() == TokenKind::Eof
    }

    fn current(&self) -> Token {
        self.lexer.current()
    }

    fn current_kind(&self) -> TokenKind {
        self.current().kind
    }

    fn current_span(&self) -> Span {
        self.current().span
    }

    fn current_symbol(&self) -> &ByteStr {
        self.current().symbol
    }

    fn current_symbol_as_bytestring(&self) -> ByteString {
        self.current_symbol().to_bytestring()
    }

    fn peek(&mut self) -> Token {
        self.lexer.peek()
    }

    fn peek_kind(&mut self) -> TokenKind {
        self.lexer.peek().kind
    }

    fn next_but_first<T>(&mut self, mut cb: impl FnMut(&mut Self) -> T) -> T {
        let result = cb(self);

        self.next();

        result
    }

    fn collect_until<T>(&mut self, kind: TokenKind, mut cb: impl FnMut(&mut Self) -> T) -> Vec<T> {
        let mut items = Vec::new();

        while self.current_kind() != kind {
            items.push(cb(self));
        }

        items
    }

    fn expect_any(&mut self, kinds: &[TokenKind]) -> Span {
        for kind in kinds {
            if self.current_kind() == *kind {
                let span = self.current_span();

                self.next();

                return span;
            }
        }

        self.expected_any_of_tokens(kinds);

        Span::missing()
    }

    fn expect(&mut self, kind: TokenKind) -> Span {
        let span = self.current_span();

        if self.is_eof() && kind != TokenKind::Eof {
            self.unexpected_end_of_file();

            Span::missing()
        } else if self.current_kind() != kind {
            self.expected_token(kind);

            Span::missing()
        } else {
            self.next();

            span
        }
    }

    fn try_consume(&mut self, kind: TokenKind) -> Option<Span> {
        match self.current_kind() {
            k if k == kind => {
                let span = self.current_span();

                self.next();

                Some(span)
            }
            _ => None,
        }
    }

    fn diagnostic(&mut self, diagnostic: ParserDiagnostic, severity: Severity, span: Span) {
        self.state
            .diagnostics
            .push(Diagnostic::new(diagnostic, severity, span));
    }
}
