use pxp_ast::{
    DocBlock, DocBlockComment, DocBlockGenericTag, DocBlockNode, DocBlockParamClosureThisTag, DocBlockParamTag, DocBlockPropertyTag, DocBlockReturnTag, DocBlockTag, DocBlockTagNode, DocBlockTextNode, DocBlockVarTag, SimpleVariable
};
use pxp_bytestring::ByteString;
use pxp_diagnostics::Severity;
use pxp_span::{Span, Spanned};
use pxp_token::TokenKind;

use crate::{Parser, ParserDiagnostic};

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

        while !self.is_eof() && self.current_kind() == TokenKind::PhpDocHorizontalWhitespace {
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
            b"@param-closure-this" | b"@phpstan-param-closure-this" => {
                self.param_closure_this_tag()
            }
            b"@param" | b"@phpstan-param" | b"@psalm-param" | b"@phan-param" => self.param_tag(),
            b"@var" | b"@phpstan-var" | b"@psalm-var" | b"@phan-var" => self.var_tag(),
            b"@return" | b"@phpstan-return" | b"@psalm-return" | b"@phan-return" | b"@phan-real-return" => self.return_tag(),
            b"@property" |
				b"@property-read" |
				b"@property-write" |
				b"@phpstan-property" |
				b"@phpstan-property-read" |
				b"@phpstan-property-write" |
				b"@psalm-property" |
				b"@psalm-property-read" |
				b"@psalm-property-write" |
				b"@phan-property" |
				b"@phan-property-read" |
				b"@phan-property-write" => self.property_tag(),
            _ => self.generic_tag(),
        };

        DocBlockTagNode {
            id: self.id(),
            span: tag.span(),
            tag,
        }
    }

    fn property_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();
        self.skip_horizontal_whitespace();

        let data_type = self.parse_optional_data_type();

        self.skip_horizontal_whitespace();

        let variable = self.parse_simple_variable();
        
        self.skip_horizontal_whitespace();

        let (text, text_span) = self.read_text_until_eol_or_close();

        let span = if let Some(text_span) = text_span {
            tag.span.join(text_span)
        } else if variable.is_missing() {
            tag.span.join(data_type.span())
        } else {
            tag.span.join(variable.span())
        };

        DocBlockTag::Property(DocBlockPropertyTag {
            id: self.id(),
            span,
            tag,
            data_type,
            variable,
            text,
        })
    }

    fn param_closure_this_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();
        self.skip_horizontal_whitespace();

        let r#type = self.parse_data_type();

        self.skip_horizontal_whitespace();

        let variable = match self.current_kind() {
            TokenKind::Variable => self.parse_simple_variable(),
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Variable],
                        found: self.current().to_owned(),
                    },
                    Severity::Warning,
                    self.current_span(),
                );

                SimpleVariable::missing(self.id(), self.current_span())
            }
        };

        self.skip_horizontal_whitespace();

        let (text, text_span) = self.read_text_until_eol_or_close();

        let span = if let Some(text_span) = text_span {
            tag.span.join(text_span)
        } else if !variable.is_missing() {
            tag.span.join(variable.span())
        } else {
            tag.span.join(r#type.span())
        };

        DocBlockTag::ParamClosureThis(DocBlockParamClosureThisTag {
            id: self.id(),
            span,
            tag,
            r#type,
            variable,
            text,
        })
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

    fn return_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();
        self.skip_horizontal_whitespace();

        let data_type = self.parse_optional_data_type();

        self.skip_horizontal_whitespace();

        let (text, text_span) = self.read_text_until_eol_or_close();

        DocBlockTag::Return(DocBlockReturnTag {
            id: self.id(),
            span: if let Some(text_span) = text_span {
                tag.span.join(text_span)
            } else if data_type.is_some() {
                tag.span.join(data_type.span())
            } else {
                tag.span
            },
            tag,
            data_type,
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
            tag,
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
            if self.is_eof() || matches!(
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
        while !self.is_eof() && self.current_kind() == TokenKind::PhpDocHorizontalWhitespace {
            self.next();
        }
    }
}
