use pxp_ast::{
    DocBlock, DocBlockComment, DocBlockGenericTag, DocBlockNode, DocBlockParamTag, DocBlockTag,
    DocBlockTagNode, DocBlockTextNode, DocBlockVarTag,
};
use pxp_bytestring::ByteString;
use pxp_span::{Span, Spanned};
use pxp_token::TokenKind;

use crate::state::State;

pub fn parse_docblock(&mut self) -> DocBlockComment {
    let current = self.current();

    if !matches!(current.kind, TokenKind::OpenPhpDoc) {
        unreachable!();
    }

    state.enter_docblock();
    self.next();
    skip_horizontal_whitespace();

    let mut nodes = Vec::new();

    loop {
        if self.is_eof() {
            break;
        }

        let current = self.current();

        if current.kind == TokenKind::ClosePhpDoc {
            self.next();
            break;
        }

        match &current.kind {
            TokenKind::PhpDocEol => {
                self.next();
            }
            TokenKind::PhpDocTag => {
                let tag = parse_docblock_tag();

                nodes.push(DocBlockNode::Tag(tag))
            }
            _ => {
                if let Some(text) = parse_docblock_text() {
                    nodes.push(DocBlockNode::Text(text))
                }
            }
        };
    }

    let span = Span::combine(current.span, nodes.span());

    state.exit_docblock();

    DocBlockComment {
        id: self.state.id(),
        span,
        doc: DocBlock {
            id: self.state.id(),
            span,
            nodes,
        },
    }
}

fn parse_docblock_tag(&mut self) -> DocBlockTagNode {
    let tag = self.current();
    let symbol = tag.symbol.as_ref().unwrap();

    let tag = match symbol.as_bytes() {
        b"@param" => param_tag(),
        b"@var" => var_tag(),
        _ => generic_tag(),
    };

    DocBlockTagNode {
        id: self.state.id(),
        span: tag.span(),
        tag,
    }
}

fn param_tag(&mut self) -> DocBlockTag {
    let tag = self.current();

    self.next();
    skip_horizontal_whitespace();

    let data_type = optional_data_type();

    skip_horizontal_whitespace();

    let variable = optional_simple_variable();

    skip_horizontal_whitespace();

    let (text, _) = read_text_until_eol_or_close();

    let previous = state.previous();
    let span = Span::combine(tag.span, previous.span);

    DocBlockTag::Param(DocBlockParamTag {
        id: self.state.id(),
        span,
        tag: tag.clone(),
        ampersand: None,
        ellipsis: None,
        data_type,
        variable,
        text,
    })
}

fn var_tag(&mut self) -> DocBlockTag {
    let tag = self.current();

    self.next();
    skip_horizontal_whitespace();

    let data_type = optional_data_type();

    skip_horizontal_whitespace();

    let variable = optional_simple_variable();

    skip_horizontal_whitespace();

    let (text, _) = read_text_until_eol_or_close();

    let previous = state.previous();
    let span = Span::combine(tag.span, previous.span);

    DocBlockTag::Var(DocBlockVarTag {
        id: self.state.id(),
        span,
        tag: tag.clone(),
        data_type,
        variable,
        text,
    })
}

fn generic_tag(&mut self) -> DocBlockTag {
    let tag = self.current();

    self.next();

    skip_horizontal_whitespace();

    let (text, text_span) = read_text_until_eol_or_close();

    let span = if let Some(text_span) = text_span {
        Span::combine(tag.span, text_span)
    } else {
        tag.span
    };

    DocBlockTag::Generic(DocBlockGenericTag {
        id: self.state.id(),
        span,
        tag: tag.clone(),
        text,
    })
}

fn parse_docblock_text(&mut self) -> Option<DocBlockTextNode> {
    let (content, span) = read_text_until_eol_or_close();

    content.as_ref()?;

    Some(DocBlockTextNode {
        id: self.state.id(),
        span: span.unwrap(),
        content: content.unwrap(),
    })
}

fn read_text_until_eol_or_close(&mut self) -> (Option<ByteString>, Option<Span>) {
    let mut text = ByteString::empty();
    let start_span = self.current_span();

    loop {
        let current = self.current();

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

        self.next();
    }

    if text.is_empty() {
        return (None, None);
    }

    let end_span = self.current_span();
    let span = Span::combine(start_span, end_span);

    (Some(text), Some(span))
}

fn skip_horizontal_whitespace(&mut self) {
    while let TokenKind::PhpDocHorizontalWhitespace = self.current_kind() {
        self.next();
    }
}
