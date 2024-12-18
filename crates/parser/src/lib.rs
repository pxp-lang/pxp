mod diagnostics;
mod expressions;
mod internal;
mod macros;
mod state;

use std::collections::HashMap;

pub use diagnostics::ParserDiagnostic;
use pxp_ast::{
    Comment, CommentKind, DocBlockComment, HashMarkComment, MultiLineComment, Name, NodeId,
    SingleLineComment, Statement, UseKind,
};
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

    id: u32,
    comments: Vec<Comment>,
    imports: HashMap<UseKind, HashMap<ByteString, ByteString>>,

    diagnostics: Vec<Diagnostic<ParserDiagnostic>>,
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
            diagnostics: parser.diagnostics.clone(),
        }
    }

    fn new(lexer: Lexer<'a>) -> Self {
        let mut imports = HashMap::new();
        imports.insert(UseKind::Normal, HashMap::new());
        imports.insert(UseKind::Function, HashMap::new());
        imports.insert(UseKind::Const, HashMap::new());

        let mut this = Self {
            lexer,
            state: State::new(),

            id: 0,
            comments: vec![],
            imports,

            diagnostics: vec![],
        };

        this.collect_comments();
        this
    }

    #[inline(always)]
    pub fn id(&mut self) -> u32 {
        self.id += 1;
        self.id
    }

    /// Return the current span and move on to the next token.
    fn next(&mut self) -> Span {
        let span = self.current_span();

        self.lexer.next();
        self.collect_comments();

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

    fn peek_kind(&mut self) -> TokenKind {
        self.lexer.peek().kind
    }

    /// Be cheeky and peek again after peeking.
    /// This isn't great for performance, but it's sometimes necessary.
    fn peek_again_kind(&mut self) -> TokenKind {
        self.lexer.peek_again().kind
    }

    fn next_but_first<T>(&mut self, mut cb: impl FnMut(&mut Self) -> T) -> T {
        let result = cb(self);

        self.next();

        result
    }

    fn skip_doc_eol(&mut self) {
        if self.current_kind() == TokenKind::PhpDocEol {
            self.next();
        }

        while self.current_kind() == TokenKind::PhpDocHorizontalWhitespace {
            self.next();
        }
    }
}
