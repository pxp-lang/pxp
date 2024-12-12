use pxp_ast::{
    BracedNamespace, BracedNamespaceBody, CommentGroup, NamespaceStatement, SimpleIdentifier,
    Statement, StatementKind, UnbracedNamespace,
};
use pxp_span::{Span, Spanned};
use pxp_token::TokenKind;

use crate::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_namespace(&mut self) -> Statement {
        let namespace = self.expect(TokenKind::Namespace);

        if self.current_kind() == TokenKind::LeftBrace {
            return self.parse_braced_namespace(namespace, None);
        }

        let name = self.parse_unqualified_or_qualified_identifier();

        match self.current_kind() {
            TokenKind::LeftBrace => self.parse_braced_namespace(namespace, Some(name)),
            _ => self.parse_unbraced_namespace(namespace, name),
        }
    }

    fn parse_braced_namespace(
        &mut self,
        namespace: Span,
        name: Option<SimpleIdentifier>,
    ) -> Statement {
        let left_brace = self.expect(TokenKind::LeftBrace);
        let statements = self.collect_until(TokenKind::RightBrace, |parser| {
            parser.parse_top_level_statement()
        });
        let right_brace = self.expect(TokenKind::RightBrace);
        let span = Span::combine(namespace, right_brace);

        Statement::new(
            self.id(),
            StatementKind::Namespace(NamespaceStatement::Braced(BracedNamespace {
                id: self.id(),
                span,
                name,
                namespace,
                body: BracedNamespaceBody {
                    id: self.id(),
                    span: Span::combine(left_brace, right_brace),
                    statements,
                    start: left_brace,
                    end: right_brace,
                },
            })),
            Span::combine(namespace, right_brace),
            CommentGroup::default(),
        )
    }

    fn parse_unbraced_namespace(&mut self, namespace: Span, name: SimpleIdentifier) -> Statement {
        let semicolon = self.expect(TokenKind::SemiColon);
        let mut statements = Vec::new();

        while self.current_kind() != TokenKind::Namespace && !self.is_eof() {
            // NOTE: If we encounter a right-brace here, it's possible that we're in a nested namespace.
            // We should check to see if the previous scope is a BracedNamespace and break out of this scope.
            if self.current_kind() == TokenKind::RightBrace {
                // FIXME: Check if the previous namespace scope is a BracedNamespace.
                todo!()
            }

            statements.push(self.parse_top_level_statement());
        }

        let span = Span::combine(namespace, statements.span());

        Statement::new(
            self.id(),
            StatementKind::Namespace(NamespaceStatement::Unbraced(UnbracedNamespace {
                id: self.id(),
                span,
                start: namespace,
                name,
                end: semicolon,
                statements,
            })),
            span,
            CommentGroup::default(),
        )
    }
}
