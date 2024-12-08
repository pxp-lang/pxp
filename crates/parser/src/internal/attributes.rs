use crate::internal::parameters;
use crate::internal::utils;
use crate::state::State;
use crate::Parser;
use pxp_ast::*;
use pxp_span::Span;
use pxp_token::TokenKind;

use super::names;

impl<'a> Parser<'a> {
    pub fn gather_attributes(&mut self) -> bool {
        if self.state.current().kind != TokenKind::Attribute {
            return false;
        }

        let start = self.state.current().span;
        let mut members = vec![];

        self.state.next();

        loop {
            let start = self.state.current().span;
            let name = self.full_name_including_self();
            let arguments = if self.state.current().kind == TokenKind::LeftParen {
                Some(self.argument_list())
            } else {
                None
            };
            let end = self.state.current().span;
            let span = Span::new(start.start, end.end);

            members.push(Attribute {
                id: self.state.id(),
                span,
                name,
                arguments,
            });

            if self.state.current().kind == TokenKind::Comma {
                self.state.next();

                if self.state.current().kind == TokenKind::RightBracket {
                    break;
                }

                continue;
            }

            break;
        }

        let end = self.skip_right_bracket();
        let span = Span::new(start.start, end.end);

        let id = self.state.id();
        self.state.attribute(AttributeGroup { id, span, members });

        // recursive, looking for multiple attribute brackets after each other.
        self.gather_attributes()
    }
}
