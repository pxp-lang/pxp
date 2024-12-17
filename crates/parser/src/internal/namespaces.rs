use crate::scoped;
use crate::state::NamespaceType;
use crate::state::Scope;
use crate::Parser;
use crate::ParserDiagnostic;
use pxp_ast::Block;
use pxp_ast::StatementKind;
use pxp_ast::*;

use pxp_diagnostics::Severity;
use pxp_span::{Span, Spanned};
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_namespace(&mut self) -> StatementKind {
        let start = self.skip(TokenKind::Namespace);
        let name = self.parse_optional_name_identifier();

        if let Some(name) = &name {
            if self.current_kind() != TokenKind::LeftBrace {
                if let Some(NamespaceType::Braced) = self.state.namespace_type() {
                    self.diagnostic(
                        ParserDiagnostic::CannotMixBracketedAndUnbracketedNamespaceDeclarations,
                        Severity::Error,
                        self.current_span(),
                    );
                }

                return self.parse_unbraced_namespace(start, name.clone());
            }
        }

        match self.state.namespace_type() {
            Some(NamespaceType::Unbraced) => {
                self.diagnostic(
                    ParserDiagnostic::CannotMixBracketedAndUnbracketedNamespaceDeclarations,
                    Severity::Error,
                    self.current_span(),
                );

                self.parse_braced_namespace(start, name)
            }
            Some(NamespaceType::Braced) if self.state.namespace().is_some() => {
                self.diagnostic(
                    ParserDiagnostic::NestedNamespace,
                    Severity::Error,
                    self.current_span(),
                );

                self.parse_braced_namespace(start, name)
            }
            _ => self.parse_braced_namespace(start, name),
        }
    }

    fn parse_unbraced_namespace(&mut self, start: Span, name: SimpleIdentifier) -> StatementKind {
        let end = self.skip_semicolon();

        let statements = scoped!(self.state, Scope::Namespace(name.symbol.clone()), {
            let mut statements = Block::new();

            while self.current_kind() != TokenKind::Namespace && !self.is_eof() {
                // NOTE: If we encounter a right-brace here, it's possible that we're in a nested namespace.
                // We should check to see if the previous scope is a BracedNamespace and break out of this scope.
                if self.current_kind() == TokenKind::RightBrace {
                    if let Some(Scope::BracedNamespace(_)) = self.state.previous_scope() {
                        break;
                    }
                }

                statements.push(self.parse_top_level_statement());
            }

            statements
        });

        StatementKind::Namespace(NamespaceStatement::Unbraced(UnbracedNamespace {
            id: self.id(),
            span: Span::combine(start, statements.span()),
            start,
            end,
            name,
            statements,
        }))
    }

    fn parse_braced_namespace(
        &mut self,
        span: Span,
        name: Option<SimpleIdentifier>,
    ) -> StatementKind {
        let body = scoped!(
            self.state,
            Scope::BracedNamespace(name.as_ref().map(|n| n.symbol.clone())),
            {
                let start = self.skip_left_brace();

                let mut statements = Block::new();
                while self.current_kind() != TokenKind::RightBrace && !self.is_eof() {
                    statements.push(self.parse_top_level_statement());
                }

                let end = self.skip_right_brace();

                BracedNamespaceBody {
                    id: self.id(),
                    span: Span::combine(start, end),
                    start,
                    end,
                    statements,
                }
            }
        );

        StatementKind::Namespace(NamespaceStatement::Braced(BracedNamespace {
            id: self.id(),
            span: Span::combine(span, body.span),
            namespace: span,
            name,
            body,
        }))
    }
}
