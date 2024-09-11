use pxp_ast::DocBlockComment;
use pxp_token::TokenKind;

use crate::state::State;

pub fn docblock(state: &mut State) -> DocBlockComment {
    let current = state.current();

    if ! matches!(current.kind, TokenKind::OpenPhpDoc) {
        unreachable!();
    }

    state.next();

    todo!();
}