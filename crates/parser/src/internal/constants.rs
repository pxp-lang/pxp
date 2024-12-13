use crate::expressions;
use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use crate::Parser;
use pxp_ast::*;
use pxp_span::Span;
use pxp_token::TokenKind;

use super::data_type::parse_data_type;
use super::names;

impl<'a> Parser<'a> {
    pub fn parse_constant(&mut self) -> ConstantStatement {
        let comments = state.comments();
        let start = self.skip(TokenKind::Const);

        let mut entries = vec![];

        loop {
            let name = names::parse_constant_identifier();
            let span = self.skip(TokenKind::Equals);
            let value = self.parse_expression();

            entries.push(ConstantEntry {
                id: self.state.id(),
                span: Span::combine(name.span, value.span),
                name,
                equals: span,
                value,
            });

            if self.current().kind == TokenKind::Comma {
                self.next();
            } else {
                break;
            }
        }

        let end = utils::skip_semicolon();
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
        let attributes = state.get_attributes();

        let comments = state.comments();
        let start = self.skip(TokenKind::Const);

        let data_type = if state.peek().kind == TokenKind::Identifier {
            Some(parse_data_type())
        } else {
            None
        };

        let mut entries = vec![];

        loop {
            let name = identifiers::parse_identifier_maybe_reserved();
            let span = self.skip(TokenKind::Equals);
            let value = self.parse_expression();

            entries.push(ClassishConstantEntry {
                id: self.state.id(),
                span: Span::combine(name.span, value.span),
                name,
                equals: span,
                value,
            });

            if self.current().kind == TokenKind::Comma {
                self.next();
            } else {
                break;
            }
        }

        let end = utils::skip_semicolon();

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
