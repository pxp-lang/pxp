use crate::expressions;
use crate::internal::attributes;
use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use pxp_ast::enums::BackedEnumBody;
use pxp_ast::enums::BackedEnumCase;
use pxp_ast::enums::BackedEnumMember;
use pxp_ast::enums::BackedEnumStatement;
use pxp_ast::enums::BackedEnumType;
use pxp_ast::enums::UnitEnumBody;
use pxp_ast::enums::UnitEnumCase;
use pxp_ast::enums::UnitEnumMember;
use pxp_ast::enums::UnitEnumStatement;
use pxp_ast::StatementKind;
use pxp_diagnostics::DiagnosticKind;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;

use super::classes::member;

pub fn parse(state: &mut State) -> StatementKind {
    let span = utils::skip(state, TokenKind::Enum);

    let name = identifiers::type_identifier(state);

    let backed_type: Option<BackedEnumType> = if state.stream.current().kind == TokenKind::Colon {
        let span = utils::skip_colon(state);
        let current = state.stream.current();

        match current.kind {
            TokenKind::Identifier => {
                let symbol = state
                    .symbol_table
                    .resolve(current.symbol.unwrap())
                    .unwrap();

                Some(match &symbol[..] {
                    b"string" => {
                        state.stream.next();
                        BackedEnumType::String(span, current.span)
                    },
                    b"int" => {
                        state.stream.next();
                        BackedEnumType::Int(span, current.span)
                    },
                    _ => {
                        state.stream.next();

                        state.diagnostic(
                            DiagnosticKind::InvalidBackedEnumType,
                            Severity::Error,
                            current.span
                        );

                        BackedEnumType::Invalid(span)
                    },
                })
            },
            TokenKind::LeftBrace => {
                state.diagnostic(
                    DiagnosticKind::UnexpectedToken { token: *current },
                    Severity::Error,
                    current.span,
                );

                Some(BackedEnumType::Invalid(span))
            },
            _ => {
                state.stream.next();

                state.diagnostic(
                    DiagnosticKind::InvalidBackedEnumType,
                    Severity::Error,
                    current.span
                );

                Some(BackedEnumType::Invalid(span))
            }
        }
    } else {
        None
    };

    let mut implements = Vec::new();
    if state.stream.current().kind == TokenKind::Implements {
        state.stream.next();

        while state.stream.current().kind != TokenKind::LeftBrace {
            implements.push(identifiers::full_type_name(state));

            if state.stream.current().kind == TokenKind::Comma {
                state.stream.next();
            } else {
                break;
            }
        }
    }

    let attributes = state.get_attributes();
    if let Some(backed_type) = backed_type {
        let body = BackedEnumBody {
            left_brace: utils::skip_left_brace(state),
            members: {
                let mut members = Vec::new();
                while state.stream.current().kind != TokenKind::RightBrace {
                    if let Some(member) = backed_member(state) {
                        members.push(member);
                    }
                }

                members
            },
            right_brace: utils::skip_right_brace(state),
        };

        StatementKind::BackedEnum(BackedEnumStatement {
            r#enum: span,
            name,
            backed_type,
            attributes,
            implements,
            body,
        })
    } else {
        let body = UnitEnumBody {
            left_brace: utils::skip_left_brace(state),
            members: {
                let mut members = Vec::new();
                while state.stream.current().kind != TokenKind::RightBrace {
                    if let Some(member) = unit_member(state) {
                        members.push(member);
                    }
                }
                members
            },
            right_brace: utils::skip_right_brace(state),
        };

        StatementKind::UnitEnum(UnitEnumStatement {
            r#enum: span,
            name,
            attributes,
            implements,
            body,
        })
    }
}

fn unit_member(state: &mut State) -> Option<UnitEnumMember> {
    let _has_attributes = attributes::gather_attributes(state);

    let current = state.stream.current();
    if current.kind == TokenKind::Case {
        let attributes = state.get_attributes();

        let start = current.span;
        state.stream.next();

        let name = identifiers::identifier_maybe_reserved(state);

        let current = state.stream.current();
        if current.kind == TokenKind::Equals {
            // parse the value, but don't do anything with it.
            let equals = utils::skip(state, TokenKind::Equals);
            let expression = expressions::create(state);
            utils::skip_semicolon(state);

            state.diagnostic(
                DiagnosticKind::UnitEnumsCannotHaveCaseValues,
                Severity::Error,
                Span::new(equals.start, expression.span.end),
            );

            return None;
        }

        let end = utils::skip_semicolon(state);

        return Some(UnitEnumMember::Case(UnitEnumCase {
            start,
            end,
            name,
            attributes,
        }));
    }

    Some(UnitEnumMember::Classish(member(state, false)))
}

fn backed_member(state: &mut State) -> Option<BackedEnumMember> {
    let _has_attributes = attributes::gather_attributes(state);

    let current = state.stream.current();
    if current.kind == TokenKind::Case {
        let attributes = state.get_attributes();

        let case = current.span;
        state.stream.next();

        let name = identifiers::identifier_maybe_reserved(state);

        let current = state.stream.current();
        if current.kind == TokenKind::SemiColon {
            // parse the semicolon, but don't do anything with it.
            let semi = utils::skip_semicolon(state);

            state.diagnostic(
                DiagnosticKind::BackedEnumCaseMustHaveValue,
                Severity::Error,
                Span::new(case.start, semi.end),
            );

            return None;
        }

        let equals = utils::skip(state, TokenKind::Equals);

        let value = expressions::create(state);

        let semicolon = utils::skip_semicolon(state);

        return Some(BackedEnumMember::Case(BackedEnumCase {
            attributes,
            case,
            name,
            equals,
            value,
            semicolon,
        }));
    }

    Some(BackedEnumMember::Classish(member(state, false)))
}
