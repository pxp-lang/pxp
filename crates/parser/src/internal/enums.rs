use crate::expressions;
use crate::internal::attributes;
use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
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
        let span = utils::skip(state, TokenKind::Enum);

        let name = names::parse_type_name(state);

        let backed_type: Option<(Span, BackedEnumType)> =
            if state.current().kind == TokenKind::Colon {
                let colon = utils::skip_colon(state);
                let current = state.current();

                match current.kind {
                    TokenKind::Identifier => {
                        let symbol = current.symbol.as_ref().unwrap();

                        Some(match &symbol[..] {
                            b"string" => {
                                state.next();
                                (colon, BackedEnumType::String(current.span))
                            }
                            b"int" => {
                                state.next();
                                (colon, BackedEnumType::Int(current.span))
                            }
                            _ => {
                                state.next();

                                state.diagnostic(
                                    ParserDiagnostic::InvalidBackedEnumType,
                                    Severity::Error,
                                    current.span,
                                );

                                (colon, BackedEnumType::Invalid)
                            }
                        })
                    }
                    TokenKind::LeftBrace => {
                        state.diagnostic(
                            ParserDiagnostic::UnexpectedToken {
                                token: current.clone(),
                            },
                            Severity::Error,
                            current.span,
                        );

                        Some((colon, BackedEnumType::Invalid))
                    }
                    _ => {
                        state.next();

                        state.diagnostic(
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
        if state.current().kind == TokenKind::Implements {
            state.next();

            while state.current().kind != TokenKind::LeftBrace {
                implements.push(names::parse_full_name(state, UseKind::Normal));

                if state.current().kind == TokenKind::Comma {
                    state.next();
                } else {
                    break;
                }
            }
        }

        let attributes = state.get_attributes();
        if let Some((colon, backed_type)) = backed_type {
            let left_brace = utils::skip_left_brace(state);
            let members = {
                let mut members = Vec::new();
                while state.current().kind != TokenKind::RightBrace {
                    if let Some(member) = parse_backed_member(state) {
                        members.push(member);
                    }
                }

                members
            };
            let right_brace = utils::skip_right_brace(state);

            let body = BackedEnumBody {
                id: state.id(),
                span: Span::combine(left_brace, right_brace),
                left_brace,
                members,
                right_brace,
            };

            StatementKind::BackedEnum(BackedEnumStatement {
                id: state.id(),
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
            let left_brace = utils::skip_left_brace(state);
            let members = {
                let mut members = Vec::new();
                while state.current().kind != TokenKind::RightBrace {
                    if let Some(member) = parse_unit_member(state) {
                        members.push(member);
                    }
                }

                members
            };
            let right_brace = utils::skip_right_brace(state);

            let body = UnitEnumBody {
                id: state.id(),
                span: Span::combine(left_brace, right_brace),
                left_brace,
                members,
                right_brace,
            };

            StatementKind::UnitEnum(UnitEnumStatement {
                id: state.id(),
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
        let _has_attributes = attributes::gather_attributes(state);

        let current = state.current();
        if current.kind == TokenKind::Case {
            let attributes = state.get_attributes();

            let start = current.span;
            state.next();

            let name = identifiers::parse_identifier_maybe_reserved(state);

            let current = state.current();
            if current.kind == TokenKind::Equals {
                // parse the value, but don't do anything with it.
                let equals = utils::skip(state, TokenKind::Equals);
                let expression = expressions::create(state);
                utils::skip_semicolon(state);

                state.diagnostic(
                    ParserDiagnostic::UnitEnumsCannotHaveCaseValues,
                    Severity::Error,
                    Span::new(equals.start, expression.span.end),
                );

                return None;
            }

            let end = utils::skip_semicolon(state);

            return Some(UnitEnumMember::Case(UnitEnumCase {
                id: state.id(),
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
        let _has_attributes = attributes::gather_attributes(state);

        let current = state.current();
        if current.kind == TokenKind::Case {
            let attributes = state.get_attributes();

            let case = current.span;
            state.next();

            let name = identifiers::parse_identifier_maybe_reserved(state);

            let current = state.current();
            if current.kind == TokenKind::SemiColon {
                // parse the semicolon, but don't do anything with it.
                let semi = utils::skip_semicolon(state);

                state.diagnostic(
                    ParserDiagnostic::BackedEnumCaseMustHaveValue,
                    Severity::Error,
                    Span::new(case.start, semi.end),
                );

                return None;
            }

            let equals = utils::skip(state, TokenKind::Equals);

            let value = expressions::create(state);

            let semicolon = utils::skip_semicolon(state);

            return Some(BackedEnumMember::Case(BackedEnumCase {
                id: state.id(),
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
