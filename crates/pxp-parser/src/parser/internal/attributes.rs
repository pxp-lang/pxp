use crate::lexer::token::TokenKind;
use crate::parser::ast::attributes::Attribute;
use crate::parser::ast::attributes::AttributeGroup;
use crate::parser::error::ParseResult;
use crate::parser::internal::identifiers;
use crate::parser::internal::parameters;
use crate::parser::internal::utils;
use crate::parser::state::State;

pub fn gather_attributes(state: &mut State) -> ParseResult<bool> {
    if state.stream.current().kind != TokenKind::Attribute {
        return Ok(false);
    }

    let start = state.stream.current().span;
    let mut members = vec![];

    state.stream.next();

    loop {
        let start = state.stream.current().span;
        let name = identifiers::full_type_name_including_self(state)?;
        let arguments = if state.stream.current().kind == TokenKind::LeftParen {
            Some(parameters::argument_list(state)?)
        } else {
            None
        };
        let end = state.stream.current().span;

        members.push(Attribute {
            start,
            name,
            arguments,
            end,
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

    let end = utils::skip_right_bracket(state)?;

    state.attribute(AttributeGroup {
        start,
        members,
        end,
    });

    // recursive, looking for multiple attribute brackets after each other.
    gather_attributes(state).map(|_| true)
}
