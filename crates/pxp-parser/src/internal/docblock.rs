use pxp_ast::{DocBlock, DocBlockComment};
use pxp_span::{Span, Spanned};
use pxp_token::TokenKind;

use crate::state::State;

pub fn docblock(state: &mut State) -> DocBlockComment {
    let current = state.current();

    if ! matches!(current.kind, TokenKind::OpenPhpDoc) {
        unreachable!();
    }

    state.next();

    let mut nodes = Vec::new();

    loop {
        if state.is_eof() {
            break;
        }

        let current = state.current();

        if current.kind == TokenKind::ClosePhpDoc {
            state.next();
            break;
        }

        // FIXME: Actually parse.
        state.next();
    }

    let span = Span::combine(current.span, nodes.span());

    DocBlockComment {
        id: state.id(),
        span,
        doc: DocBlock {
            id: state.id(),
            span,
            nodes,
        }
    }
}