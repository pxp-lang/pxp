use crate::lexer::token::TokenKind;
use crate::parser::ast::constant::ClassishConstant;
use crate::parser::ast::constant::ConstantEntry;
use crate::parser::ast::constant::ConstantStatement;
use crate::parser::ast::modifiers::ConstantModifierGroup;
use crate::parser::error::ParseResult;
use crate::parser::expressions;
use crate::parser::internal::identifiers;
use crate::parser::internal::utils;
use crate::parser::state::State;

pub fn parse(state: &mut State) -> ParseResult<ConstantStatement> {
    let comments = state.stream.comments();
    let start = utils::skip(state, TokenKind::Const)?;

    let mut entries = vec![];

    loop {
        let name = identifiers::constant_identifier(state)?;
        let span = utils::skip(state, TokenKind::Equals)?;
        let value = expressions::create(state)?;

        entries.push(ConstantEntry {
            name,
            equals: span,
            value,
        });

        if state.stream.current().kind == TokenKind::Comma {
            state.stream.next();
        } else {
            break;
        }
    }

    let end = utils::skip_semicolon(state)?;

    Ok(ConstantStatement {
        comments,
        r#const: start,
        entries,
        semicolon: end,
    })
}

pub fn classish(
    state: &mut State,
    modifiers: ConstantModifierGroup,
) -> ParseResult<ClassishConstant> {
    let attributes = state.get_attributes();

    let comments = state.stream.comments();
    let start = utils::skip(state, TokenKind::Const)?;

    let mut entries = vec![];

    loop {
        let name = identifiers::identifier_maybe_reserved(state)?;
        let span = utils::skip(state, TokenKind::Equals)?;
        let value = expressions::create(state)?;

        entries.push(ConstantEntry {
            name,
            equals: span,
            value,
        });

        if state.stream.current().kind == TokenKind::Comma {
            state.stream.next();
        } else {
            break;
        }
    }

    let end = utils::skip_semicolon(state)?;

    Ok(ClassishConstant {
        comments,
        attributes,
        modifiers,
        r#const: start,
        entries,
        semicolon: end,
    })
}
