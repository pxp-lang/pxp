use pxp_ast::{DocBlock, DocBlockComment, DocBlockGenericTag, DocBlockNode, DocBlockTag, DocBlockTagNode, DocBlockTextNode};
use pxp_bytestring::ByteString;
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

        match &current.kind {
            TokenKind::PhpDocEol => {
                state.next();
            }
            TokenKind::PhpDocTag => {
                let tag = docblock_tag(state);

                nodes.push(DocBlockNode::Tag(tag))
            }
            _ => {
                if let Some(text) = docblock_text(state) {
                    nodes.push(DocBlockNode::Text(text))
                }
            }
        };
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
    let tag = state.current();

    state.next();

    let (text, span) = match read_text_until_eol_or_close(state) {
        Some((text, text_span)) => (Some(text), Span::combine(tag.span, text_span)),
        None => (None, tag.span),
    };

    DocBlockTagNode {
        id: state.id(),
        span,
        tag: DocBlockTag::Generic(DocBlockGenericTag {
            id: state.id(),
            span: tag.span,
            tag: tag.clone(),
            text,
        })
    }
}

fn docblock_text(state: &mut State) -> Option<DocBlockTextNode> {
    let (content, span) = read_text_until_eol_or_close(state)?;

    Some(DocBlockTextNode {
        id: state.id(),
        span,
        content,
    })
}

fn read_text_until_eol_or_close(state: &mut State) -> Option<(ByteString, Span)> {
    let mut text = ByteString::empty();
    let start_span = state.current().span;

    loop {
        let current = state.current();

        if matches!(current.kind, TokenKind::PhpDocEol | TokenKind::ClosePhpDoc) {
            break;
        }

        let bytes = match &current.symbol {
            Some(symbol) => symbol.clone(),
            None => {
                let string = current.kind.to_string();
                ByteString::new(string.as_bytes().to_vec())
            }
        };

        text.extend_with_bytes(&bytes);

        state.next();
    }

    if text.is_empty() {
        return None;
    }

    let end_span = state.current().span;
    let span = Span::combine(start_span, end_span);

    Some((text, span))
}
