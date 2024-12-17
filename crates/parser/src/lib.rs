mod diagnostics;
mod expressions;
mod internal;
mod macros;
mod state;

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
        let mut this = Self {
            lexer,
            state: State::new(),

            id: 0,
            comments: vec![],
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

    fn diagnostic(&mut self, diagnostic: ParserDiagnostic, severity: Severity, span: Span) {
        self.state
            .diagnostics
            .push(Diagnostic::new(diagnostic, severity, span));
    }

    fn skip_doc_eol(&mut self) {
        if self.current_kind() == TokenKind::PhpDocEol {
            self.next();
        }

        while self.current_kind() == TokenKind::PhpDocHorizontalWhitespace {
            self.next();
        }
    }

    pub fn maybe_resolve_identifier(&self, id: NodeId, token: &Token, kind: UseKind) -> Name {
        let part = match &token.kind {
            TokenKind::Identifier | TokenKind::Enum | TokenKind::From => {
                token.symbol.to_bytestring()
            }
            TokenKind::QualifiedIdentifier => token.symbol.before_first(b'\\').to_bytestring(),
            _ if self.is_soft_reserved_identifier(token.kind) => token.symbol.to_bytestring(),
            _ => unreachable!("{:?}", token.kind),
        };

        let map = self.state.imports.get(&kind).unwrap();

        // We found an import that matches the first part of the identifier, so we can resolve it.
        if let Some(imported) = map.get(&part) {
            match &token.kind {
                TokenKind::Identifier | TokenKind::From | TokenKind::Enum => Name::resolved(
                    id,
                    imported.clone(),
                    token.symbol.to_bytestring(),
                    token.span,
                ),
                TokenKind::QualifiedIdentifier => {
                    // Qualified identifiers might be aliased, so we need to take the full un-aliased import and
                    // concatenate that with everything after the first part of the qualified identifier.
                    let bytestring = token.symbol.to_bytestring();
                    let parts = bytestring.splitn(2, |c| *c == b'\\').collect::<Vec<_>>();
                    let rest = parts[1].to_vec().into();
                    let coagulated = imported.coagulate(&[rest], Some(b"\\"));

                    Name::resolved(id, coagulated, bytestring, token.span)
                }
                _ => unreachable!(),
            }
        // We didn't find an import, but since we're trying to resolve the name of a class like, we can
        // follow PHP's name resolution rules and just prepend the current namespace.
        //
        // Additionally, if the name we're trying to resolve is qualified, then PHP's name resolution rules say that
        // we should just prepend the current namespace if the import map doesn't contain the first part.
        } else if kind == UseKind::Normal || token.kind == TokenKind::QualifiedIdentifier {
            Name::resolved(
                id,
                self.state
                    .join_with_namespace(&token.symbol.to_bytestring()),
                token.symbol.to_bytestring(),
                token.span,
            )
        // Unqualified names in the global namespace can be resolved without any imports, since we can
        // only be referencing something else inside of the global namespace.
        } else if (kind == UseKind::Function || kind == UseKind::Const)
            && token.kind == TokenKind::Identifier
            && self.state.namespace().is_none()
        {
            Name::resolved(
                id,
                token.symbol.to_bytestring(),
                token.symbol.to_bytestring(),
                token.span,
            )
        } else {
            Name::unresolved(
                id,
                token.symbol.to_bytestring(),
                token.kind.into(),
                token.span,
            )
        }
    }
}
