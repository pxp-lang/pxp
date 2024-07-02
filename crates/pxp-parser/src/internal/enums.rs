use crate::expressions;
use crate::internal::attributes;
use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use crate::ParserDiagnostic;
use pxp_ast::*;
use pxp_ast::StatementKind;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_syntax::backed_enum_type::BackedEnumType;
use pxp_token::TokenKind;

use super::classes::member;
use super::names;

pub fn parse(state: &mut State) -> StatementKind {
    let span = utils::skip(state, TokenKind::Enum);

    let name = names::type_name(state);

    let backed_type: Option<BackedEnumType> = if state.stream.current().kind == TokenKind::Colon {
        let span = utils::skip_colon(state);
        let current = state.stream.current();

        match current.kind {
            TokenKind::Identifier => {
                let symbol = state.symbol_table.resolve(current.symbol.unwrap()).unwrap();

                Some(match &symbol[..] {
                    b"string" => {
                        state.stream.next();
                        BackedEnumType::String(span, current.span)
                    }
                    b"int" => {
                        state.stream.next();
                        BackedEnumType::Int(span, current.span)
                    }
                    _ => {
                        state.stream.next();

                        state.diagnostic(
                            ParserDiagnostic::InvalidBackedEnumType,
                            Severity::Error,
                            current.span,
                        );

                        BackedEnumType::Invalid(span)
                    }
                })
            }
            TokenKind::LeftBrace => {
                state.diagnostic(
                    ParserDiagnostic::UnexpectedToken { token: *current },
                    Severity::Error,
                    current.span,
                );

                Some(BackedEnumType::Invalid(span))
            }
            _ => {
                state.stream.next();

                state.diagnostic(
                    ParserDiagnostic::InvalidBackedEnumType,
                    Severity::Error,
                    current.span,
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
            implements.push(names::full_name(state, UseKind::Normal));

            if state.stream.current().kind == TokenKind::Comma {
                state.stream.next();
            } else {
                break;
            }
        }
    }

    let attributes = state.get_attributes();
    if let Some(backed_type) = backed_type {
        let left_brace = utils::skip_left_brace(state);
        let members = {
            let mut members = Vec::new();
            while state.stream.current().kind != TokenKind::RightBrace {
                if let Some(member) = backed_member(state) {
                    members.push(member);
                }
            }

            members
        };
        let right_brace = utils::skip_right_brace(state);

        let body = BackedEnumBody {
            span: Span::combine(left_brace, right_brace),
            left_brace,
            members,
            right_brace,
        };

        StatementKind::BackedEnum(BackedEnumStatement {
            span: Span::combine(span, body.span),
            r#enum: span,
            name,
            backed_type,
            attributes,
            implements,
            body,
        })
    } else {
        let left_brace = utils::skip_left_brace(state);
        let members = {
            let mut members = Vec::new();
            while state.stream.current().kind != TokenKind::RightBrace {
                if let Some(member) = unit_member(state) {
                    members.push(member);
                }
            }

            members
        };
        let right_brace = utils::skip_right_brace(state);

        let body = UnitEnumBody {
            span: Span::combine(left_brace, right_brace),
            left_brace,
            members,
            right_brace,
        };

        StatementKind::UnitEnum(UnitEnumStatement {
            span: Span::combine(span, body.span),
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
                ParserDiagnostic::UnitEnumsCannotHaveCaseValues,
                Severity::Error,
                Span::new(equals.start, expression.span.end),
            );

            return None;
        }

        let end = utils::skip_semicolon(state);

        return Some(UnitEnumMember::Case(UnitEnumCase {
            span: Span::combine(start, end),
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
            span: Span::combine(case, semicolon),
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
