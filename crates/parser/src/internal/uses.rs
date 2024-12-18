use crate::Parser;
use crate::internal::diagnostics::ParserDiagnostic;
use pxp_ast::GroupUseStatement;
use pxp_ast::StatementKind;
use pxp_ast::Use;
use pxp_ast::UseKind;
use pxp_ast::UseStatement;
use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_use_statement(&mut self) -> StatementKind {
        let r#use = self.current_span();

        self.next();

        let kind = match self.current_kind() {
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

        if self.peek_kind() == TokenKind::LeftBrace {
            let prefix = self.parse_full_name_identifier();
            let prefix_symbol = prefix.symbol.clone();

            self.next();

            let mut uses = Vec::new();
            while self.current_kind() != TokenKind::RightBrace {
                let start_span = self.current_span();
                let use_kind = match self.current_kind() {
                    TokenKind::Function => {
                        if kind != UseKind::Normal {
                            self.diagnostic(
                                ParserDiagnostic::MixedImportTypes,
                                Severity::Error,
                                self.current_span(),
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
                                self.current_span(),
                            );
                        }

                        self.next();
                        Some(UseKind::Const)
                    }
                    _ => None,
                };

                let name = self.parse_full_type_name_identifier();
                let mut alias = None;
                if self.current_kind() == TokenKind::As {
                    self.next();
                    alias = Some(self.parse_type_identifier());
                }

                let symbol = name.symbol.clone();
                let alias_symbol = alias.as_ref().map(|a| a.symbol.clone());
                let import_kind = use_kind.unwrap_or(kind);
                let span = if alias.is_some() {
                    Span::combine(start_span, alias.span())
                } else {
                    start_span
                };

                uses.push(Use {
                    id: self.id(),
                    span,
                    name: Name::resolved(
                        self.id(),
                        prefix_symbol
                            .clone()
                            .coagulate(&[name.symbol.clone()], Some(b"\\")),
                        name.symbol,
                        name.span,
                    ),
                    kind: use_kind.unwrap_or(kind),
                    alias,
                });

                self.add_prefixed_import(&import_kind, prefix_symbol.clone(), symbol, alias_symbol);

                if self.current_kind() == TokenKind::Comma {
                    self.next();
                    continue;
                }
            }

            self.skip_right_brace();
            let semicolon = self.skip_semicolon();

            StatementKind::GroupUse(GroupUseStatement {
                id: self.id(),
                span: Span::combine(prefix.span, semicolon),
                prefix,
                kind,
                uses,
            })
        } else {
            let mut uses = Vec::new();
            while !self.is_eof() {
                let start_span = self.current_span();
                let name = self.parse_use_name();
                let mut alias = None;
                if self.current_kind() == TokenKind::As {
                    self.next();
                    alias = Some(self.parse_type_identifier());
                }

                let alias_symbol = alias.as_ref().map(|a| a.symbol.clone());
                let span = if alias.is_some() {
                    Span::combine(start_span, alias.span())
                } else {
                    start_span
                };

                uses.push(Use {
                    id: self.id(),
                    span,
                    name: name.clone(),
                    kind,
                    alias,
                });

                self.add_import(&kind, name.symbol().clone(), alias_symbol);

                if self.current_kind() == TokenKind::Comma {
                    self.next();
                    continue;
                }

                self.skip_semicolon();
                break;
            }

            let span = Span::combine(r#use, uses.span());

            StatementKind::Use(UseStatement {
                id: self.id(),
                span,
                uses,
                kind,
            })
        }
    }
}
