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
        };

        this.collect_comments();
        this
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

    fn peek(&mut self) -> Token {
        self.lexer.peek()
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

    fn skip_doc_eol(&mut self) {
        if self.current_kind() == TokenKind::PhpDocEol {
            self.next();
        }

        while self.current_kind() == TokenKind::PhpDocHorizontalWhitespace {
            self.next();
        }
    }

    fn collect_comments(&mut self) {
        loop {
            if self.is_eof() {
                break;
            }

            if !matches!(
                self.current_kind(),
                TokenKind::SingleLineComment
                    | TokenKind::MultiLineComment
                    | TokenKind::HashMarkComment
                    | TokenKind::DocBlockComment
                    | TokenKind::OpenPhpDoc,
            ) {
                break;
            }

            let id = self.state.id();
            let comment_id = self.state.id();

            let (comment, move_forward) = match self.current_kind() {
                TokenKind::SingleLineComment => (
                    Comment {
                        id,
                        span: self.current_span(),
                        kind: CommentKind::SingleLine(SingleLineComment {
                            id: comment_id,
                            span: self.current_span(),
                            content: self.current_symbol_as_bytestring(),
                        }),
                    },
                    true,
                ),
                TokenKind::MultiLineComment => (
                    Comment {
                        id,
                        span: self.current_span(),
                        kind: CommentKind::MultiLine(MultiLineComment {
                            id: comment_id,
                            span: self.current_span(),
                            content: self.current_symbol_as_bytestring(),
                        }),
                    },
                    true,
                ),
                TokenKind::HashMarkComment => (
                    Comment {
                        id,
                        span: self.current_span(),
                        kind: CommentKind::HashMark(HashMarkComment {
                            id: comment_id,
                            span: self.current_span(),
                            content: self.current_symbol_as_bytestring(),
                        }),
                    },
                    true,
                ),
                #[cfg(not(feature = "docblocks"))]
                TokenKind::DocBlockComment => (
                    Comment {
                        id,
                        span: self.current_span(),
                        kind: CommentKind::DocBlock(DocBlockComment {
                            id: comment_id,
                            span: self.current_span(),
                            content: self.current_symbol_as_bytestring(),
                        }),
                    },
                    true,
                ),
                #[cfg(feature = "docblocks")]
                TokenKind::OpenPhpDoc => {
                    let docblock = crate::internal::docblock::parse_docblock(self);

                    (
                        Comment {
                            id,
                            span: docblock.span,
                            kind: CommentKind::DocBlock(docblock),
                        },
                        false,
                    )
                }
                _ => unreachable!(),
            };

            self.state.comments.push(comment);

            if move_forward {
                self.next();
            }
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
