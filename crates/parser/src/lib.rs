use pxp_ast::{NodeId, Statement};
use pxp_bytestring::ByteStr;
use pxp_diagnostics::{Diagnostic, Severity};
use pxp_lexer::Lexer;

mod internal;
mod diagnostics;

pub use diagnostics::ParserDiagnostic;
use pxp_span::Span;
use pxp_token::{Token, TokenKind};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    diagnostics: Vec<Diagnostic<ParserDiagnostic>>,

    id: NodeId,
}

#[derive(Debug, Clone)]
pub struct ParseResult {
    pub ast: Vec<Statement>,
    pub diagnostics: Vec<Diagnostic<ParserDiagnostic>>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        Self { id: 0, lexer, diagnostics: Vec::new() }
    }

    pub fn parse(lexer: Lexer<'a>) -> ParseResult {
        let mut parser = Self::new(lexer);

        parser.parse_program()
    }

    fn parse_program(&mut self) -> ParseResult {
        let mut ast = Vec::new();

        while ! self.is_eof() {
            ast.push(self.parse_top_level_statement());
        }

        ParseResult { ast, diagnostics: self.diagnostics.clone() }
    }

    fn id(&mut self) -> NodeId {
        self.id += 1;
        self.id
    }

    fn is_eof(&self) -> bool {
        self.current_kind() == TokenKind::Eof
    }

    fn current_kind(&self) -> TokenKind {
        self.lexer.current().kind
    }

    fn current(&self) -> Token {
        self.lexer.current()
    }

    fn current_span(&self) -> Span {
        self.lexer.current().span
    }

    fn current_symbol(&self) -> &ByteStr {
        self.lexer.current().symbol
    }

    fn peek_kind(&mut self) -> TokenKind {
        self.lexer.peek().kind
    }

    fn peek_span(&mut self) -> Span {
        self.lexer.peek().span
    }

    fn peek_symbol(&mut self) -> &ByteStr {
        self.lexer.peek().symbol
    }

    /// Consume the current token after executing the callback.
    fn next_but_first<T>(&mut self, mut cb: impl FnMut(&mut Self) -> T) -> T {
        let result = cb(self);

        self.next();

        result
    }

    /// Consume the current token.
    fn next(&mut self) {
        self.lexer.next();
    }

    /// Collect a series of nodes into a `Vec` until the given `TokenKind` is encountered.
    fn collect_until<T>(&mut self, kind: TokenKind, mut cb: impl FnMut(&mut Self) -> T) -> Vec<T> {
        let mut items = Vec::new();

        while self.current_kind() != kind {
            items.push(cb(self));
        }

        items
    }

    /// Expect any of the given `TokenKind` values.
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

    /// Expect the given `TokenKind`.
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

    pub(crate) fn unexpected_end_of_file(&mut self) {
        self.diagnostic(ParserDiagnostic::UnexpectedEndOfFile, Severity::Error, self.current_span());
    }

    pub(crate) fn expected_token(&mut self, kind: TokenKind) {
        self.diagnostic(ParserDiagnostic::ExpectedToken { expected: kind, found: self.current().to_owned() }, Severity::Error, self.current_span());
    }

    pub(crate) fn expected_any_of_tokens(&mut self, kinds: &[TokenKind]) {
        self.diagnostic(ParserDiagnostic::ExpectedOneOfTokens { expected: kinds.to_vec(), found: self.current().to_owned() }, Severity::Error, self.current_span());
    }

    /// Optionally consume the given `TokenKind`.
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
        self.diagnostics.push(Diagnostic::new(diagnostic, severity, span));
    }
}
