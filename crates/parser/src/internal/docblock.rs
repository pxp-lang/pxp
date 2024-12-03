use pxp_ast::{
    DocBlock, DocBlockComment, DocBlockGenericTag, DocBlockNode, DocBlockParamTag, DocBlockTag,
    DocBlockTagNode, DocBlockTextNode, DocBlockVarTag,
};
use pxp_bytestring::ByteString;
use pxp_span::{Span, Spanned};
use pxp_token::TokenKind;

use crate::state::State;

use super::{
    data_type::optional_data_type,
    variables::{optional_simple_variable, simple_variable},
};

pub fn docblock(state: &mut State) -> DocBlockComment {
    let current = state.current();

    if !matches!(current.kind, TokenKind::OpenPhpDoc) {
        unreachable!();
    }

    state.enter_docblock();
    state.next();
    skip_horizontal_whitespace(state);

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

    state.exit_docblock();

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
    let symbol = tag.symbol.as_ref().unwrap();

    let tag = match symbol.as_bytes() {
        b"@param" => param_tag(state),
        b"@var" => var_tag(state),
        _ => generic_tag(state),
    };

    DocBlockTagNode {
        id: state.id(),
        span: tag.span(),
        tag,
    }
}

fn param_tag(state: &mut State) -> DocBlockTag {
    let tag = state.current();

    state.next();
    skip_horizontal_whitespace(state);

    let data_type = optional_data_type(state);

    skip_horizontal_whitespace(state);

    let variable = optional_simple_variable(state);

    skip_horizontal_whitespace(state);

    let (text, _) = read_text_until_eol_or_close(state);

    let previous = state.previous();
    let span = Span::combine(tag.span, previous.span);

    DocBlockTag::Param(DocBlockParamTag {
        id: state.id(),
        span,
        tag: tag.clone(),
        data_type,
        variable,
        text,
    })
}

fn var_tag(state: &mut State) -> DocBlockTag {
    let tag = state.current();

    state.next();
    skip_horizontal_whitespace(state);

    let data_type = optional_data_type(state);

    skip_horizontal_whitespace(state);

    let variable = optional_simple_variable(state);

    skip_horizontal_whitespace(state);

    let (text, _) = read_text_until_eol_or_close(state);

    let previous = state.previous();
    let span = Span::combine(tag.span, previous.span);

    DocBlockTag::Var(DocBlockVarTag {
        id: state.id(),
        span,
        tag: tag.clone(),
        data_type,
        variable,
        text,
    })
}

fn generic_tag(state: &mut State) -> DocBlockTag {
    let tag = state.current();

    state.next();

    skip_horizontal_whitespace(state);

    let (text, text_span) = read_text_until_eol_or_close(state);

    let span = if text_span.is_some() {
        Span::combine(tag.span, text_span.unwrap())
    } else {
        tag.span
    };

    DocBlockTag::Generic(DocBlockGenericTag {
        id: state.id(),
        span,
        tag: tag.clone(),
        text,
    })
}

fn docblock_text(state: &mut State) -> Option<DocBlockTextNode> {
    let (content, span) = read_text_until_eol_or_close(state);

    if content.is_none() {
        return None;
    }

    Some(DocBlockTextNode {
        id: state.id(),
        span: span.unwrap(),
        content: content.unwrap(),
    })
}

fn read_text_until_eol_or_close(state: &mut State) -> (Option<ByteString>, Option<Span>) {
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
        return (None, None);
    }

    let end_span = state.current().span;
    let span = Span::combine(start_span, end_span);

    (Some(text), Some(span))
}

fn skip_horizontal_whitespace(state: &mut State) {
    while let TokenKind::PhpDocHorizontalWhitespace = state.current().kind {
        state.next();
    }
}
