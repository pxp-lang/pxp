use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use crate::Parser;
use crate::ParserDiagnostic;
use pxp_ast::GroupUseStatement;
use pxp_ast::StatementKind;
use pxp_ast::Use;
use pxp_ast::UseKind;
use pxp_ast::UseStatement;
use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;

use super::names;

impl<'a> Parser<'a> {
    pub fn parse_use_statement(&mut self) -> StatementKind {
        let r#use = self.current().span;

        self.next();

        let kind = match self.current().kind {
            TokenKind::Function => {
                self.next();
                UseKind::Function
            }
            TokenKind::Const => {
                self.next();
                UseKind::Const
            }
            _ => UseKind::Normal,
        };

        if state.peek().kind == TokenKind::LeftBrace {
            let prefix = identifiers::parse_full_name_identifier();
            let prefix_symbol = prefix.symbol.clone();

            self.next();

            let mut uses = Vec::new();
            while self.current().kind != TokenKind::RightBrace {
                let start_span = self.current().span;
                let use_kind = match self.current().kind {
                    TokenKind::Function => {
                        if kind != UseKind::Normal {
                            self.diagnostic(
                                ParserDiagnostic::MixedImportTypes,
                                Severity::Error,
                                self.current().span,
                            );
                        }

                        self.next();
                        Some(UseKind::Function)
                    }
                    TokenKind::Const => {
                        if kind != UseKind::Normal {
                            self.diagnostic(
                                ParserDiagnostic::MixedImportTypes,
                                Severity::Error,
                                self.current().span,
                            );
                        }

                        self.next();
                        Some(UseKind::Const)
                    }
                    _ => None,
                };

                let name = identifiers::parse_full_type_name_identifier();
                let mut alias = None;
                if self.current().kind == TokenKind::As {
                    self.next();
                    alias = Some(identifiers::parse_type_identifier());
                }

                let symbol = name.symbol.clone();
                let alias_symbol = alias.as_ref().map(|a| a.symbol.clone());
                let import_kind = use_kind.unwrap_or(kind);
                let end_span = state.previous().span;

                uses.push(Use {
                    id: self.state.id(),
                    span: Span::combine(start_span, end_span),
                    name: Name::resolved(
                        self.state.id(),
                        prefix_symbol
                            .clone()
                            .coagulate(&[name.symbol.clone()], Some(b"\\")),
                        name.symbol,
                        name.span,
                    ),
                    kind: use_kind,
                    alias,
                });

                state.add_prefixed_import(
                    &import_kind,
                    prefix_symbol.clone(),
                    symbol,
                    alias_symbol,
                );

                if self.current().kind == TokenKind::Comma {
                    self.next();
                    continue;
                }
            }

            utils::skip_right_brace();
            let semicolon = utils::skip_semicolon();

            StatementKind::GroupUse(GroupUseStatement {
                id: self.state.id(),
                span: Span::combine(prefix.span, semicolon),
                prefix,
                kind,
                uses,
            })
        } else {
            let mut uses = Vec::new();
            while !state.is_eof() {
                let start_span = self.current().span;
                let name = names::parse_use_name();
                let mut alias = None;
                if self.current().kind == TokenKind::As {
                    self.next();
                    alias = Some(identifiers::parse_type_identifier());
                }

                let alias_symbol = alias.as_ref().map(|a| a.symbol.clone());
                let end_span = state.previous().span;

                uses.push(Use {
                    id: self.state.id(),
                    span: Span::combine(start_span, end_span),
                    name: name.clone(),
                    kind: None,
                    alias,
                });

                state.add_import(&kind, name.symbol().clone(), alias_symbol);

                if self.current().kind == TokenKind::Comma {
                    self.next();
                    continue;
                }

                utils::skip_semicolon();
                break;
            }

            let span = Span::combine(r#use, state.previous().span);

            StatementKind::Use(UseStatement {
                id: self.state.id(),
                span,
                uses,
                kind,
            })
        }
    }
}
