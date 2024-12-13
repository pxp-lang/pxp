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
        if self.current().kind != TokenKind::Attribute {
            return false;
        }

        let start = self.current().span;
        let mut members = vec![];

        self.next();

        loop {
            let start = self.current().span;
            let name = names::parse_full_name_including_self();
            let arguments = if self.current().kind == TokenKind::LeftParen {
                Some(parameters::parse_argument_list())
            } else {
                None
            };
            let end = self.current().span;
            let span = Span::new(start.start, end.end);

            members.push(Attribute {
                id: self.state.id(),
                span,
                name,
                arguments,
            });

            if self.current().kind == TokenKind::Comma {
                self.next();

                if self.current().kind == TokenKind::RightBracket {
                    break;
                }

                continue;
            }

            break;
        }

        let end = utils::skip_right_bracket();
        let span = Span::new(start.start, end.end);

        let id = self.state.id();
        state.attribute(AttributeGroup { id, span, members });

        // recursive, looking for multiple attribute brackets after each other.
        gather_attributes()
    }
}
