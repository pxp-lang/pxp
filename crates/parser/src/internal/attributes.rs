use crate::Parser;
use pxp_ast::*;
use pxp_span::Span;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn gather_attributes(&mut self) -> bool {
        if self.current_kind() != TokenKind::Attribute {
            return false;
        }

        let start = self.current_span();
        let mut members = vec![];

        self.next();

        loop {
            let start = self.current_span();
            let name = self.parse_full_name_including_self();
            let arguments = if self.current_kind() == TokenKind::LeftParen {
                Some(self.parse_argument_list())
            } else {
                None
            };
            let end = self.current_span();
            let span = Span::new(start.start, end.end);

            members.push(Attribute {
                id: self.id(),
                span,
                name,
                arguments,
            });

            if self.current_kind() == TokenKind::Comma {
                self.next();

                if self.current_kind() == TokenKind::RightBracket {
                    break;
                }

                continue;
            }

            break;
        }

        let end = self.skip_right_bracket();
        let span = Span::new(start.start, end.end);

        let id = self.id();
        self.state.attribute(AttributeGroup { id, span, members });

        // recursive, looking for multiple attribute brackets after each other.
        self.gather_attributes()
    }
}
