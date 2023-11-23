use crate::internal::identifiers;
use crate::internal::parameters;
use crate::internal::utils;
use crate::state::State;
use pxp_ast::attributes::Attribute;
use pxp_ast::attributes::AttributeGroup;
use pxp_span::Span;
use pxp_token::TokenKind;

pub fn gather_attributes(state: &mut State) -> bool {
    if state.stream.current().kind != TokenKind::Attribute {
        return false;
    }

    let start = state.stream.current().span;
    let mut members = vec![];

    state.stream.next();

    loop {
        let start = state.stream.current().span;
        let name = identifiers::full_type_name_including_self(state);
        let arguments = if state.stream.current().kind == TokenKind::LeftParen {
            Some(parameters::argument_list(state))
        } else {
            None
        };
        let end = state.stream.current().span;
        let span = Span::new(start.start, end.end);

        members.push(Attribute {
            span,
            name,
            arguments,
        });

        if state.stream.current().kind == TokenKind::Comma {
            state.stream.next();

            if state.stream.current().kind == TokenKind::RightBracket {
                break;
            }

            continue;
        }

        break;
    }

    let end = utils::skip_right_bracket(state);
    let span = Span::new(start.start, end.end);

    state.attribute(AttributeGroup {
        span,
        members,
    });

    // recursive, looking for multiple attribute brackets after each other.
    gather_attributes(state) || true
}
