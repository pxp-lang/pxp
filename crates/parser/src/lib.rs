use crate::internal::attributes;
use crate::internal::blocks;
use crate::internal::classes;
use crate::internal::constants;
use crate::internal::control_flow;
use crate::internal::enums;
use crate::internal::functions;
use crate::internal::goto;
use crate::internal::identifiers;
use crate::internal::interfaces;
use crate::internal::loops;
use crate::internal::namespaces;
use crate::internal::traits;
use crate::internal::try_block;
use crate::internal::uses;
use crate::internal::utils;
use crate::internal::variables;
use crate::state::State;
use internal::literals::parse_literal;
use pxp_ast::Statement;
use pxp_ast::*;
use pxp_ast::{StatementKind, StaticVar};
use pxp_bytestring::ByteStr;
use pxp_bytestring::ByteString;
use pxp_diagnostics::Diagnostic;
use pxp_diagnostics::Severity;
use pxp_lexer::Lexer;
use pxp_span::Span;
use pxp_span::Spanned;

use pxp_token::OpenTagKind;
use pxp_token::Token;
use pxp_token::TokenKind;

use pxp_ast::ClosingTagStatement;
use pxp_ast::EchoOpeningTagStatement;
use pxp_ast::EchoStatement;
use pxp_ast::ExpressionStatement;
use pxp_ast::FullOpeningTagStatement;
use pxp_ast::GlobalStatement;
use pxp_ast::HaltCompilerStatement;
use pxp_ast::InlineHtmlStatement;
use pxp_ast::ReturnStatement;
use pxp_ast::ShortOpeningTagStatement;
use pxp_ast::StaticStatement;

mod diagnostics;
mod expressions;
mod internal;
mod macros;
mod state;

pub use diagnostics::ParserDiagnostic;

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
    pub fn new(lexer: Lexer<'a>) -> Self {
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
