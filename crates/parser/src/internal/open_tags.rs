use pxp_ast::{CommentGroup, Statement, StatementKind};
use pxp_token::{OpenTagKind, TokenKind};

use crate::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_open_tag(&mut self) -> Statement {
        match self.current_kind() {
            TokenKind::OpenTag(OpenTagKind::Full) => self.parse_full_open_tag(),
            _ => unreachable!()
        }
    }

    fn parse_full_open_tag(&mut self) -> Statement {
        let span = self.expect(TokenKind::OpenTag(OpenTagKind::Full));

        Statement::new(
            self.id(),
            StatementKind::FullOpeningTag(pxp_ast::FullOpeningTagStatement { id: self.id(), span: span }),
            span,
            CommentGroup::default()
        )
    }
}
