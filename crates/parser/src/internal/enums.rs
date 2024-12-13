use crate::expressions;
use crate::internal::attributes;
use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use crate::Parser;
use crate::ParserDiagnostic;
use pxp_ast::StatementKind;
use pxp_ast::*;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;

use super::classes::parse_classish_member;
use super::names;

impl<'a> Parser<'a> {
    pub fn parse_enum(&mut self) -> StatementKind {
        let span = self.skip(TokenKind::Enum);

        let name = names::parse_type_name();

        let backed_type: Option<(Span, BackedEnumType)> =
            if self.current().kind == TokenKind::Colon {
                let colon = utils::skip_colon();
                let current = self.current();

                match current.kind {
                    TokenKind::Identifier => {
                        let symbol = current.symbol.as_ref().unwrap();

                        Some(match &symbol[..] {
                            b"string" => {
                                self.next();
                                (colon, BackedEnumType::String(current.span))
                            }
                            b"int" => {
                                self.next();
                                (colon, BackedEnumType::Int(current.span))
                            }
                            _ => {
                                self.next();

                                self.diagnostic(
                                    ParserDiagnostic::InvalidBackedEnumType,
                                    Severity::Error,
                                    current.span,
                                );

                                (colon, BackedEnumType::Invalid)
                            }
                        })
                    }
                    TokenKind::LeftBrace => {
                        self.diagnostic(
                            ParserDiagnostic::UnexpectedToken {
                                token: current.clone(),
                            },
                            Severity::Error,
                            current.span,
                        );

                        Some((colon, BackedEnumType::Invalid))
                    }
                    _ => {
                        self.next();

                        self.diagnostic(
                            ParserDiagnostic::InvalidBackedEnumType,
                            Severity::Error,
                            current.span,
                        );

                        Some((colon, BackedEnumType::Invalid))
                    }
                }
            } else {
                None
            };

        let mut implements = Vec::new();
        if self.current().kind == TokenKind::Implements {
            self.next();

            while self.current().kind != TokenKind::LeftBrace {
                implements.push(names::parse_full_name(state, UseKind::Normal));

                if self.current().kind == TokenKind::Comma {
                    self.next();
                } else {
                    break;
                }
            }
        }

        let attributes = state.get_attributes();
        if let Some((colon, backed_type)) = backed_type {
            let left_brace = utils::skip_left_brace();
            let members = {
                let mut members = Vec::new();
                while self.current().kind != TokenKind::RightBrace {
                    if let Some(member) = parse_backed_member() {
                        members.push(member);
                    }
                }

                members
            };
            let right_brace = utils::skip_right_brace();

            let body = BackedEnumBody {
                id: self.state.id(),
                span: Span::combine(left_brace, right_brace),
                left_brace,
                members,
                right_brace,
            };

            StatementKind::BackedEnum(BackedEnumStatement {
                id: self.state.id(),
                span: Span::combine(span, body.span),
                r#enum: span,
                name,
                colon,
                backed_type,
                attributes,
                implements,
                body,
            })
        } else {
            let left_brace = utils::skip_left_brace();
            let members = {
                let mut members = Vec::new();
                while self.current().kind != TokenKind::RightBrace {
                    if let Some(member) = parse_unit_member() {
                        members.push(member);
                    }
                }

                members
            };
            let right_brace = utils::skip_right_brace();

            let body = UnitEnumBody {
                id: self.state.id(),
                span: Span::combine(left_brace, right_brace),
                left_brace,
                members,
                right_brace,
            };

            StatementKind::UnitEnum(UnitEnumStatement {
                id: self.state.id(),
                span: Span::combine(span, body.span),
                r#enum: span,
                name,
                attributes,
                implements,
                body,
            })
        }
    }

    fn parse_unit_member(&mut self) -> Option<UnitEnumMember> {
        let _has_attributes = attributes::gather_attributes();

        let current = self.current();
        if current.kind == TokenKind::Case {
            let attributes = state.get_attributes();

            let start = current.span;
            self.next();

            let name = identifiers::parse_identifier_maybe_reserved();

            let current = self.current();
            if current.kind == TokenKind::Equals {
                // parse the value, but don't do anything with it.
                let equals = self.skip(TokenKind::Equals);
                let expression = self.parse_expression();
                utils::skip_semicolon();

                self.diagnostic(
                    ParserDiagnostic::UnitEnumsCannotHaveCaseValues,
                    Severity::Error,
                    Span::new(equals.start, expression.span.end),
                );

                return None;
            }

            let end = utils::skip_semicolon();

            return Some(UnitEnumMember::Case(UnitEnumCase {
                id: self.state.id(),
                span: Span::combine(start, end),
                start,
                end,
                name,
                attributes,
            }));
        }

        Some(UnitEnumMember::Classish(parse_classish_member(
            state, false,
        )))
    }

    fn parse_backed_member(&mut self) -> Option<BackedEnumMember> {
        let _has_attributes = attributes::gather_attributes();

        let current = self.current();
        if current.kind == TokenKind::Case {
            let attributes = state.get_attributes();

            let case = current.span;
            self.next();

            let name = identifiers::parse_identifier_maybe_reserved();

            let current = self.current();
            if current.kind == TokenKind::SemiColon {
                // parse the semicolon, but don't do anything with it.
                let semi = utils::skip_semicolon();

                self.diagnostic(
                    ParserDiagnostic::BackedEnumCaseMustHaveValue,
                    Severity::Error,
                    Span::new(case.start, semi.end),
                );

                return None;
            }

            let equals = self.skip(TokenKind::Equals);

            let value = self.parse_expression();

            let semicolon = utils::skip_semicolon();

            return Some(BackedEnumMember::Case(BackedEnumCase {
                id: self.state.id(),
                span: Span::combine(case, semicolon),
                attributes,
                case,
                name,
                equals,
                value,
                semicolon,
            }));
        }

        Some(BackedEnumMember::Classish(parse_classish_member(
            state, false,
        )))
    }
}
