use pxp_ast::{DocBlock, DocBlockComment, DocBlockNode, DocBlockTagNode, DocBlockTextNode};
use pxp_span::{Span, Spanned};
use pxp_token::TokenKind;

use crate::state::State;

pub fn docblock(state: &mut State) -> DocBlockComment {
    let current = state.current();

    if !matches!(current.kind, TokenKind::OpenPhpDoc) {
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

        let node = match &current.kind {
            TokenKind::PhpDocTag => {
                let tag = docblock_tag(state);

                DocBlockNode::Tag(tag)
            },
            _ => {
                let text = docblock_text(state);

                DocBlockNode::Text(text)
            },
        };

        nodes.push(node);
    }

    let span = Span::combine(current.span, nodes.span());

    DocBlockComment {
        id: state.id(),
        span,
        doc: DocBlock {
            id: state.id(),
            span,
            nodes,
        },
    }
}

fn docblock_tag(state: &mut State) -> DocBlockTagNode {
    todo!()
}

fn docblock_text(state: &mut State) -> DocBlockTextNode {
    todo!()
}