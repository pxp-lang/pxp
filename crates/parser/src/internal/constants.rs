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
        let start = utils::skip(state, TokenKind::Const);

        let mut entries = vec![];

        loop {
            let name = names::parse_constant_identifier(state);
            let span = utils::skip(state, TokenKind::Equals);
            let value = expressions::create(state);

            entries.push(ConstantEntry {
                id: state.id(),
                span: Span::combine(name.span, value.span),
                name,
                equals: span,
                value,
            });

            if state.current().kind == TokenKind::Comma {
                state.next();
            } else {
                break;
            }
        }

        let end = utils::skip_semicolon(state);
        let span = Span::combine(start, end);

        ConstantStatement {
            id: state.id(),
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
        let start = utils::skip(state, TokenKind::Const);

        let data_type = if state.peek().kind == TokenKind::Identifier {
            Some(parse_data_type(state))
        } else {
            None
        };

        let mut entries = vec![];

        loop {
            let name = identifiers::parse_identifier_maybe_reserved(state);
            let span = utils::skip(state, TokenKind::Equals);
            let value = expressions::create(state);

            entries.push(ClassishConstantEntry {
                id: state.id(),
                span: Span::combine(name.span, value.span),
                name,
                equals: span,
                value,
            });

            if state.current().kind == TokenKind::Comma {
                state.next();
            } else {
                break;
            }
        }

        let end = utils::skip_semicolon(state);

        ClassishConstant {
            id: state.id(),
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
