use pxp_ast::{
    DocBlock, DocBlockComment, DocBlockGenericTag, DocBlockNode, DocBlockParamTag, DocBlockTag,
    DocBlockTagNode, DocBlockTextNode, DocBlockVarTag,
};
use pxp_bytestring::ByteString;
use pxp_span::{Span, Spanned};
use pxp_token::TokenKind;

use crate::Parser;

impl<'a> Parser<'a> {
    pub(crate) const fn is_in_docblock(&self) -> bool {
        self.in_docblock
    }

    fn enter_docblock(&mut self) {
        self.in_docblock = true;
    }

    fn exit_docblock(&mut self) {
        self.in_docblock = false;
    }

    pub(crate) fn skip_doc_eol(&mut self) {
        if self.current_kind() == TokenKind::PhpDocEol {
            self.next();
        }

        while self.current_kind() == TokenKind::PhpDocHorizontalWhitespace {
            self.next();
        }
    }

    pub(crate) fn parse_docblock(&mut self) -> DocBlockComment {
        if self.current_kind() != TokenKind::OpenPhpDoc {
            unreachable!();
        }

        self.enter_docblock();

        let start = self.next();

        self.skip_horizontal_whitespace();

        let mut nodes = Vec::new();

        while !self.is_eof() && self.current_kind() != TokenKind::ClosePhpDoc {
            match self.current_kind() {
                TokenKind::PhpDocEol => {
                    self.next();
                }
                TokenKind::PhpDocTag => {
                    let tag = self.parse_docblock_tag();

                    nodes.push(DocBlockNode::Tag(tag))
                }
                _ => {
                    if let Some(text) = self.parse_docblock_text() {
                        nodes.push(DocBlockNode::Text(text))
                    }
                }
            };
        }

        let close_phpdoc = self.skip(TokenKind::ClosePhpDoc);
        let span = start.join(close_phpdoc);

        self.exit_docblock();

        DocBlockComment {
            id: self.id(),
            span,
            doc: DocBlock {
                id: self.id(),
                span,
                nodes,
            },
        }
    }

    fn parse_docblock_tag(&mut self) -> DocBlockTagNode {
        let tag = match self.current_symbol().as_ref() {
            b"@param" => self.param_tag(),
            b"@var" => self.var_tag(),
            _ => self.generic_tag(),
        };

        DocBlockTagNode {
            id: self.id(),
            span: tag.span(),
            tag,
        }
    }

    fn param_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();
        self.skip_horizontal_whitespace();

        let data_type = self.parse_optional_data_type();

        self.skip_horizontal_whitespace();

        let variable = self.parse_optional_simple_variable();

        self.skip_horizontal_whitespace();

        let (text, text_span) = self.read_text_until_eol_or_close();

        let span = if let Some(text_span) = text_span {
            tag.span.join(text_span)
        } else if variable.is_some() {
            tag.span.join(variable.span())
        } else if data_type.is_some() {
            tag.span.join(data_type.span())
        } else {
            tag.span
        };

        DocBlockTag::Param(DocBlockParamTag {
            id: self.id(),
            span,
            tag,
            ampersand: None,
            ellipsis: None,
            data_type,
            variable,
            text,
        })
    }

    fn var_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();
        self.skip_horizontal_whitespace();

        let data_type = self.parse_optional_data_type();

        self.skip_horizontal_whitespace();

        let variable = self.parse_optional_simple_variable();

        self.skip_horizontal_whitespace();

        let (text, text_span) = self.read_text_until_eol_or_close();

        let span = if let Some(text_span) = text_span {
            tag.span.join(text_span)
        } else if variable.is_some() {
            tag.span.join(variable.span())
        } else if data_type.is_some() {
            tag.span.join(data_type.span())
        } else {
            tag.span
        };

        DocBlockTag::Var(DocBlockVarTag {
            id: self.id(),
            span,
            tag,
            data_type,
            variable,
            text,
        })
    }

    fn generic_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();
        self.skip_horizontal_whitespace();

        let (text, text_span) = self.read_text_until_eol_or_close();

        let span = if let Some(text_span) = text_span {
            Span::combine(tag.span, text_span)
        } else {
            tag.span
        };

        DocBlockTag::Generic(DocBlockGenericTag {
            id: self.id(),
            span,
            tag: tag.clone(),
            text,
        })
    }

    fn parse_docblock_text(&mut self) -> Option<DocBlockTextNode> {
        let (content, span) = self.read_text_until_eol_or_close();

        content.as_ref()?;

        Some(DocBlockTextNode {
            id: self.id(),
            span: span.unwrap(),
            content: content.unwrap(),
        })
    }

    fn read_text_until_eol_or_close(&mut self) -> (Option<ByteString>, Option<Span>) {
        let mut text = ByteString::empty();
        let start_span = self.current_span();

        loop {
            if matches!(
                self.current_kind(),
                TokenKind::PhpDocEol | TokenKind::ClosePhpDoc
            ) {
                break;
            }

            text.extend_with_bytes(self.current_symbol());

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
}
