use crate::lexer::token::Span;
use crate::lexer::token::TokenKind;
use crate::parser::ast::enums::BackedEnumBody;
use crate::parser::ast::enums::BackedEnumCase;
use crate::parser::ast::enums::BackedEnumMember;
use crate::parser::ast::enums::BackedEnumStatement;
use crate::parser::ast::enums::BackedEnumType;
use crate::parser::ast::enums::UnitEnumBody;
use crate::parser::ast::enums::UnitEnumCase;
use crate::parser::ast::enums::UnitEnumMember;
use crate::parser::ast::enums::UnitEnumStatement;
use crate::parser::ast::functions::ConcreteMethod;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::ast::Statement;
use crate::parser::error;
use crate::parser::error::ParseResult;
use crate::parser::expressions;
use crate::parser::internal::attributes;
use crate::parser::internal::constants;
use crate::parser::internal::functions;
use crate::parser::internal::functions::Method;
use crate::parser::internal::identifiers;
use crate::parser::internal::modifiers;
use crate::parser::internal::utils;
use crate::parser::state::State;

use super::traits;

pub fn parse(state: &mut State) -> ParseResult<Statement> {
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

        Ok(Statement::BackedEnum(BackedEnumStatement {
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

        Ok(Statement::UnitEnum(UnitEnumStatement {
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
    let has_attributes = attributes::gather_attributes(state)?;

    if !has_attributes && state.stream.current().kind == TokenKind::Use {
        return traits::usage(state)
            .map(UnitEnumMember::TraitUsage)
            .map(Some);
    }

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

    let modifiers = modifiers::collect(state)?;

    if state.stream.current().kind == TokenKind::Const {
        return constants::classish(state, modifiers::constant_group(modifiers)?)
            .map(UnitEnumMember::Constant)
            .map(Some);
    }

    method(state, modifiers, enum_name).map(|method| method.map(UnitEnumMember::Method))
}

fn backed_member(
    state: &mut State,
    enum_name: &SimpleIdentifier,
) -> ParseResult<Option<BackedEnumMember>> {
    let has_attributes = attributes::gather_attributes(state)?;

    if !has_attributes && state.stream.current().kind == TokenKind::Use {
        return traits::usage(state)
            .map(BackedEnumMember::TraitUsage)
            .map(Some);
    }

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

    let modifiers = modifiers::collect(state)?;

    if state.stream.current().kind == TokenKind::Const {
        return constants::classish(state, modifiers::constant_group(modifiers)?)
            .map(BackedEnumMember::Constant)
            .map(Some);
    }

    method(state, modifiers, enum_name).map(|method| method.map(BackedEnumMember::Method))
}

fn method(
    state: &mut State,
    modifiers: Vec<(Span, TokenKind)>,
    enum_name: &SimpleIdentifier,
) -> ParseResult<Option<ConcreteMethod>> {
    let method = functions::method(
        state,
        functions::MethodType::Concrete,
        modifiers::enum_method_group(modifiers)?,
        Some(enum_name),
    )?;

    match method {
        Method::ConcreteConstructor(constructor) => {
            let error = error::constructor_in_enum(state, enum_name, &constructor.name);

            state.record(error);

            Ok(None)
        }
        Method::Concrete(method) => {
            match method.name.value[..].to_ascii_lowercase().as_slice() {
                b"__get" | b"__set" | b"__serialize" | b"__unserialize" | b"__destruct"
                | b"__wakeup" | b"__sleep" | b"__set_state" | b"__unset" | b"__isset"
                | b"__debuginfo" | b"__clone" | b"__tostring" => {
                    let error = error::magic_method_in_enum(state, enum_name, &method.name);

                    state.record(error);
                }
                _ => {}
            }

            Ok(Some(method))
        }
        Method::Abstract(_) | Method::AbstractConstructor(_) => unreachable!(),
    }
}
