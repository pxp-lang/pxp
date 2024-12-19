use crate::internal::diagnostics::ParserDiagnostic;
use crate::scoped;
use crate::Parser;
use pxp_ast::Block;
use pxp_ast::StatementKind;
use pxp_ast::*;

use pxp_bytestring::ByteString;
use pxp_diagnostics::Severity;
use pxp_span::{Span, Spanned};
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub(crate) fn parse_namespace(&mut self) -> StatementKind {
        let start = self.skip(TokenKind::Namespace);
        let name = self.parse_optional_name_identifier();

        if let Some(name) = &name {
            if self.current_kind() != TokenKind::LeftBrace {
                if let Some(NamespaceType::Braced) = self.namespace_type() {
                    self.diagnostic(
                        ParserDiagnostic::CannotMixBracketedAndUnbracketedNamespaceDeclarations,
                        Severity::Error,
                        self.current_span(),
                    );
                }

                return self.parse_unbraced_namespace(start, name.clone());
            }
        }

        match self.namespace_type() {
            Some(NamespaceType::Unbraced) => {
                self.diagnostic(
                    ParserDiagnostic::CannotMixBracketedAndUnbracketedNamespaceDeclarations,
                    Severity::Error,
                    self.current_span(),
                );

                self.parse_braced_namespace(start, name)
            }
            Some(NamespaceType::Braced) if self.namespace().is_some() => {
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

        let statements = scoped!(self, Scope::Namespace(name.symbol.clone()), {
            let mut statements = Block::new();

            while self.current_kind() != TokenKind::Namespace && !self.is_eof() {
                // NOTE: If we encounter a right-brace here, it's possible that we're in a nested namespace.
                // We should check to see if the previous scope is a BracedNamespace and break out of this scope.
                if self.current_kind() == TokenKind::RightBrace {
                    if let Some(Scope::BracedNamespace(_)) = self.previous_scope() {
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
            self,
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

    fn namespace_type(&self) -> Option<&NamespaceType> {
        self.namespace_type.as_ref()
    }

    pub(crate) fn namespace(&self) -> Option<&Scope> {
        self.stack.iter().next()
    }

    pub(crate) fn strip_leading_namespace_qualifier(&mut self, symbol: &ByteString) -> ByteString {
        if symbol.starts_with(b"\\") {
            ByteString::from(&symbol[1..])
        } else {
            symbol.clone()
        }
    }

    pub(crate) fn join_with_namespace(&self, name: &ByteString) -> ByteString {
        match self.namespace() {
            Some(Scope::Namespace(namespace)) => namespace.coagulate(&[name.clone()], Some(b"\\")),
            Some(Scope::BracedNamespace(Some(namespace))) => {
                namespace.coagulate(&[name.clone()], Some(b"\\"))
            }
            _ => name.clone(),
        }
    }

    pub(crate) fn previous_scope(&self) -> Option<&Scope> {
        self.stack.get(self.stack.len() - 2)
    }

    pub(crate) fn enter(&mut self, scope: Scope) {
        match &scope {
            Scope::Namespace(_) => {
                self.namespace_type = Some(NamespaceType::Unbraced);
            }
            Scope::BracedNamespace(_) => {
                self.namespace_type = Some(NamespaceType::Braced);
            }
        }

        self.stack.push_back(scope);
    }

    pub(crate) fn exit(&mut self) {
        self.stack.pop_back();
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scope {
    Namespace(ByteString),
    BracedNamespace(Option<ByteString>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NamespaceType {
    Braced,
    Unbraced,
}
