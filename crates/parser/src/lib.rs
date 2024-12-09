use pxp_ast::Statement;
use pxp_bytestring::ByteStr;
use pxp_lexer::Lexer;

mod internal;
mod diagnostics;

pub use diagnostics::ParserDiagnostic;
use pxp_span::Span;
use pxp_token::TokenKind;

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    diagnostics: Vec<ParserDiagnostic>,
}

#[derive(Debug, Clone)]
pub struct ParseResult {
    pub ast: Vec<Statement>,
    pub diagnostics: Vec<ParserDiagnostic>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer, diagnostics: Vec::new() }
    }

    pub fn parse(lexer: Lexer<'a>) -> ParseResult {
        let mut parser = Self::new(lexer);

        parser.parse_program()
    }

    fn parse_program(&mut self) -> ParseResult {
        let mut ast = Vec::new();

        while self.current_kind() != TokenKind::Eof {
            ast.push(self.parse_top_level_statement());
        }

        ParseResult { ast, diagnostics: self.diagnostics.clone() }
    }

    fn current_kind(&self) -> TokenKind {
        self.lexer.current().kind
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

    fn next(&mut self) {
        self.lexer.next();
    }
}
