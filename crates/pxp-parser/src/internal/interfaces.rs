use crate::internal::utils;
use crate::state::State;
use pxp_ast::StatementKind;
use pxp_ast::UseKind;
use pxp_ast::*;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

use super::classes::member;
use super::names;

pub fn parse(state: &mut State) -> StatementKind {
    let span = utils::skip(state, TokenKind::Interface);

    let name = names::type_name(state);

    let current = state.stream.current();
    let extends = if current.kind == TokenKind::Extends {
        let span = current.span;

        state.stream.next();

        let parents = utils::at_least_one_comma_separated_no_trailing::<Name>(state, &|state| {
            names::full_name(state, UseKind::Normal)
        });

        Some(InterfaceExtends {
            id: state.id(),
            span: Span::combine(span, parents.span()),
            extends: span,
            parents,
        })
    } else {
        None
    };

    let attributes = state.get_attributes();

    let left_brace = utils::skip_left_brace(state);
    let members = {
        let mut members = Vec::new();
        while state.stream.current().kind != TokenKind::RightBrace {
            members.push(member(state, true));
        }

        members
    };
    let right_brace = utils::skip_right_brace(state);

    let body = InterfaceBody {
        id: state.id(),
        span: Span::combine(left_brace, right_brace),
        left_brace,
        members,
        right_brace,
    };

    StatementKind::Interface(InterfaceStatement {
        id: state.id(),
        span: Span::combine(span, body.span),
        interface: span,
        name,
        attributes,
        extends,
        body,
    })
}
