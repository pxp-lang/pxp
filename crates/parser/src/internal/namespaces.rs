use crate::internal::identifiers;
use crate::internal::utils;
use crate::scoped;
use crate::state::NamespaceType;
use crate::state::Scope;
use crate::state::State;
use crate::Parser;
use crate::ParserDiagnostic;
use pxp_ast::Block;
use pxp_ast::StatementKind;
use pxp_ast::*;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_namespace(&mut self) -> StatementKind {
        let start = self.skip(TokenKind::Namespace);
        let name = self.parse_optional_name_identifier();

        let current = self.current();

        if let Some(name) = &name {
            if current.kind != TokenKind::LeftBrace {
                if let Some(NamespaceType::Braced) = state.namespace_type() {
                    self.diagnostic(
                        ParserDiagnostic::CannotMixBracketedAndUnbracketedNamespaceDeclarations,
                        Severity::Error,
                        current.span,
                    );
                }

                return parse_unbraced_namespace(start, name.clone());
            }
        }

        match state.namespace_type() {
            Some(NamespaceType::Unbraced) => {
                self.diagnostic(
                    ParserDiagnostic::CannotMixBracketedAndUnbracketedNamespaceDeclarations,
                    Severity::Error,
                    current.span,
                );

                parse_braced_namespace(start, name)
            }
            Some(NamespaceType::Braced) if state.namespace().is_some() => {
                self.diagnostic(
                    ParserDiagnostic::NestedNamespace,
                    Severity::Error,
                    current.span,
                );

                parse_braced_namespace(start, name)
            }
            _ => parse_braced_namespace(start, name),
        }
    }

    fn parse_unbraced_namespace(&mut self, start: Span, name: SimpleIdentifier) -> StatementKind {
        let end = self.skip_semicolon();

        let statements = scoped!(Scope::Namespace(name.symbol.clone()), {
            let mut statements = Block::new();

            while self.current_kind() != TokenKind::Namespace && !self.is_eof() {
                // NOTE: If we encounter a right-brace here, it's possible that we're in a nested namespace.
                // We should check to see if the previous scope is a BracedNamespace and break out of this scope.
                if self.current_kind() == TokenKind::RightBrace {
                    if let Some(Scope::BracedNamespace(_)) = state.previous_scope() {
                        break;
                    }
                }

                statements.push(crate::top_level_statement());
            }

            statements
        });

        StatementKind::Namespace(NamespaceStatement::Unbraced(UnbracedNamespace {
            id: self.state.id(),
            span: Span::combine(start, statements.span()),
            start,
            end,
            name: Name::resolved(
                self.state.id(),
                name.symbol.clone(),
                name.symbol.clone(),
                name.span,
            ),
            statements,
        }))
    }

    fn parse_braced_namespace(
        &mut self,
        span: Span,
        name: Option<SimpleIdentifier>,
    ) -> StatementKind {
        let body = scoped!(
            state,
            Scope::BracedNamespace(name.as_ref().map(|n| n.symbol.clone())),
            {
                let start = self.skip_left_brace();

                let mut statements = Block::new();
                while self.current_kind() != TokenKind::RightBrace && !self.is_eof() {
                    statements.push(crate::top_level_statement());
                }

                let end = self.skip_right_brace();

                BracedNamespaceBody {
                    id: self.state.id(),
                    span: Span::combine(start, end),
                    start,
                    end,
                    statements,
                }
            }
        );

        StatementKind::Namespace(NamespaceStatement::Braced(BracedNamespace {
            id: self.state.id(),
            span: Span::combine(span, body.span),
            namespace: span,
            name: name
                .map(|n| Name::resolved(self.state.id(), n.symbol.clone(), n.symbol.clone(), n.span)),
            body,
        }))
    }
}
