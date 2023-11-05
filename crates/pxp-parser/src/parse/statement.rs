use pxp_ast::{Statement, StatementKind, ExpressionStatement};
use pxp_span::HasSpan;
use pxp_token::TokenKind;

use crate::state::ParserState;

use super::{create, utils::skip_semicolon};

pub fn statement(state: &mut ParserState) -> Statement {
    // FIXME: Gather attributes here and emit errors at some point if the subsequent statement does not accept attributes.
    let has_attributes = false;

    let current = state.stream.current();
    let peek = state.stream.peek();

    if has_attributes {
        todo!();
    } else {
        match current.kind() {
            TokenKind::FullOpenTag => {
                let span = current.span();
                state.stream.next();

                Statement::new(StatementKind::FullOpenTag, span)
            },
            _ => {
                let expression = create(state);
                skip_semicolon(state);
                let span = expression.span;

                Statement {
                    kind: StatementKind::Expression(ExpressionStatement { expression }),
                    span
                }
            },
        }
    }
}