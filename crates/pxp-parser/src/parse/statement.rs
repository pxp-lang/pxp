use pxp_ast::{Statement, StatementKind};
use pxp_span::HasSpan;
use pxp_token::TokenKind;

use crate::state::ParserState;

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
            _ => todo!(),
        }
    }
}