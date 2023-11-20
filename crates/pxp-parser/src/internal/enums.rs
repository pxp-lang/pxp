use crate::error;
use crate::error::ParseResult;
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
use pxp_ast::identifiers::SimpleIdentifier;
use pxp_ast::StatementKind;
use pxp_token::TokenKind;

use super::classes::member;

pub fn parse(state: &mut State) -> ParseResult<StatementKind> {
    let span = utils::skip(state, TokenKind::Enum)?;

    let name = identifiers::type_identifier(state)?;

    let backed_type: Option<BackedEnumType> = if state.stream.current().kind == TokenKind::Colon {
        let span = utils::skip_colon(state)?;

        let identifier = identifiers::identifier_of(state, &["string", "int"])?;
        Some(match &identifier.value[..] {
            b"string" => BackedEnumType::String(span, identifier.span),
            b"int" => BackedEnumType::Int(span, identifier.span),
            _ => unreachable!(),
        })
    } else {
        None
    };

    let mut implements = Vec::new();
    if state.stream.current().kind == TokenKind::Implements {
        state.stream.next();

        while state.stream.current().kind != TokenKind::LeftBrace {
            implements.push(identifiers::full_type_name(state)?);

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
            left_brace: utils::skip_left_brace(state)?,
            members: {
                let mut members = Vec::new();
                while state.stream.current().kind != TokenKind::RightBrace {
                    if let Some(member) = backed_member(state, &name)? {
                        members.push(member);
                    }
                }

                members
            },
            right_brace: utils::skip_right_brace(state)?,
        };

        Ok(StatementKind::BackedEnum(BackedEnumStatement {
            r#enum: span,
            name,
            backed_type,
            attributes,
            implements,
            body,
        }))
    } else {
        let body = UnitEnumBody {
            left_brace: utils::skip_left_brace(state)?,
            members: {
                let mut members = Vec::new();
                while state.stream.current().kind != TokenKind::RightBrace {
                    if let Some(member) = unit_member(state, &name)? {
                        members.push(member);
                    }
                }
                members
            },
            right_brace: utils::skip_right_brace(state)?,
        };

        Ok(StatementKind::UnitEnum(UnitEnumStatement {
            r#enum: span,
            name,
            attributes,
            implements,
            body,
        }))
    }
}

fn unit_member(
    state: &mut State,
    enum_name: &SimpleIdentifier,
) -> ParseResult<Option<UnitEnumMember>> {
    let _has_attributes = attributes::gather_attributes(state)?;

    let current = state.stream.current();
    if current.kind == TokenKind::Case {
        let attributes = state.get_attributes();

        let start = current.span;
        state.stream.next();

        let name = identifiers::identifier_maybe_reserved(state)?;

        let current = state.stream.current();
        if current.kind == TokenKind::Equals {
            // parse the value, but don't do anything with it.
            let _ = utils::skip(state, TokenKind::Equals)?;
            let _ = expressions::create(state)?;
            let _ = utils::skip_semicolon(state)?;

            let error = error::case_value_for_unit_enum(state, enum_name, &name, current.span);

            state.record(error);

            return Ok(None);
        }

        let end = utils::skip_semicolon(state)?;

        return Ok(Some(UnitEnumMember::Case(UnitEnumCase {
            start,
            end,
            name,
            attributes,
        })));
    }

    Ok(Some(UnitEnumMember::Classish(member(
        state, false, enum_name,
    )?)))
}

fn backed_member(
    state: &mut State,
    enum_name: &SimpleIdentifier,
) -> ParseResult<Option<BackedEnumMember>> {
    let _has_attributes = attributes::gather_attributes(state)?;

    let current = state.stream.current();
    if current.kind == TokenKind::Case {
        let attributes = state.get_attributes();

        let case = current.span;
        state.stream.next();

        let name = identifiers::identifier_maybe_reserved(state)?;

        let current = state.stream.current();
        if current.kind == TokenKind::SemiColon {
            // parse the semicolon, but don't do anything with it.
            let _ = utils::skip_semicolon(state)?;

            let error =
                error::missing_case_value_for_backed_enum(state, enum_name, &name, current.span);

            state.record(error);

            return Ok(None);
        }

        let equals = utils::skip(state, TokenKind::Equals)?;

        let value = expressions::create(state)?;

        let semicolon = utils::skip_semicolon(state)?;

        return Ok(Some(BackedEnumMember::Case(BackedEnumCase {
            attributes,
            case,
            name,
            equals,
            value,
            semicolon,
        })));
    }

    Ok(Some(BackedEnumMember::Classish(member(
        state, false, enum_name,
    )?)))
}
