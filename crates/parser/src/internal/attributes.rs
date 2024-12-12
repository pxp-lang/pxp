use crate::internal::parameters;
use crate::internal::utils;
use crate::state::State;
use pxp_ast::*;
use pxp_span::Span;
use pxp_token::TokenKind;

use super::names;

pub fn gather_attributes(state: &mut State) -> bool {
    if state.current().kind != TokenKind::Attribute {
        return false;
    }

    let start = state.current().span;
    let mut members = vec![];

    state.next();

    loop {
        let start = state.current().span;
        let name = names::parse_full_name_including_self(state);
        let arguments = if state.current().kind == TokenKind::LeftParen {
            Some(parameters::parse_argument_list(state))
        } else {
            None
        };
        let end = state.current().span;
        let span = Span::new(start.start, end.end);

        members.push(Attribute {
            id: state.id(),
            span,
            name,
            arguments,
        });

        if state.current().kind == TokenKind::Comma {
            state.next();

            if state.current().kind == TokenKind::RightBracket {
                break;
            }

            continue;
        }

        break;
    }

    let end = utils::skip_right_bracket(state);
    let span = Span::new(start.start, end.end);

    let id = state.id();
    state.attribute(AttributeGroup { id, span, members });

    // recursive, looking for multiple attribute brackets after each other.
    gather_attributes(state)
}
