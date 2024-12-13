use crate::Parser;
use pxp_ast::*;
use pxp_span::Span;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_constant(&mut self) -> ConstantStatement {
        let comments = self.state.comments();
        let start = self.skip(TokenKind::Const);

        let mut entries = vec![];

        loop {
            let name = self.parse_constant_identifier();
            let span = self.skip(TokenKind::Equals);
            let value = self.parse_expression();

            entries.push(ConstantEntry {
                id: self.state.id(),
                span: Span::combine(name.span, value.span),
                name,
                equals: span,
                value,
            });

            if self.current_kind() == TokenKind::Comma {
                self.next();
            } else {
                break;
            }
        }

        let end = self.skip_semicolon();
        let span = Span::combine(start, end);

        ConstantStatement {
            id: self.state.id(),
            span,
            comments,
            r#const: start,
            entries,
            semicolon: end,
        }
    }

    pub fn parse_classish_constant(
        &mut self,
        modifiers: ConstantModifierGroup,
    ) -> ClassishConstant {
        let attributes = self.state.get_attributes();
        let comments = self.state.comments();
        let start = self.skip(TokenKind::Const);

        let data_type = if self.peek_kind() == TokenKind::Identifier {
            Some(self.parse_data_type())
        } else {
            None
        };

        let mut entries = vec![];

        loop {
            let name = self.parse_identifier_maybe_reserved();
            let span = self.skip(TokenKind::Equals);
            let value = self.parse_expression();

            entries.push(ClassishConstantEntry {
                id: self.state.id(),
                span: Span::combine(name.span, value.span),
                name,
                equals: span,
                value,
            });

            if self.current_kind() == TokenKind::Comma {
                self.next();
            } else {
                break;
            }
        }

        let end = self.skip_semicolon();

        ClassishConstant {
            id: self.state.id(),
            span: if !modifiers.is_empty() {
                Span::combine(modifiers.span, end)
            } else {
                Span::combine(start, end)
            },
            comments,
            attributes,
            modifiers,
            r#const: start,
            data_type,
            entries,
            semicolon: end,
        }
    }
}
