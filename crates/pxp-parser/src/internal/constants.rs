use crate::expressions;
use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use pxp_ast::*;
use pxp_token::TokenKind;

use super::data_type::data_type;

pub fn parse(state: &mut State) -> ConstantStatement {
    let comments = state.stream.comments();
    let start = utils::skip(state, TokenKind::Const);

    let mut entries = vec![];

    loop {
        let name = identifiers::constant_identifier(state);
        let span = utils::skip(state, TokenKind::Equals);
        let value = expressions::create(state);

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

    let end = utils::skip_semicolon(state);

    ConstantStatement {
        comments,
        r#const: start,
        entries,
        semicolon: end,
    }
}

pub fn classish(state: &mut State, modifiers: ConstantModifierGroup) -> ClassishConstant {
    let attributes = state.get_attributes();

    let comments = state.stream.comments();
    let start = utils::skip(state, TokenKind::Const);

    let data_type = if state.stream.peek().kind == TokenKind::Identifier {
        Some(data_type(state))
    } else {
        None
    };

    let mut entries = vec![];

    loop {
        let name = identifiers::identifier_maybe_reserved(state);
        let span = utils::skip(state, TokenKind::Equals);
        let value = expressions::create(state);

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

    let end = utils::skip_semicolon(state);

    ClassishConstant {
        comments,
        attributes,
        modifiers,
        r#const: start,
        data_type,
        entries,
        semicolon: end,
    }
}
