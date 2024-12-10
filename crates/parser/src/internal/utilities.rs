use pxp_span::Span;
use pxp_token::TokenKind;

use crate::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn optional_comma(&mut self) -> bool {
        match self.current_kind() {
            TokenKind::Comma => {
                self.expect(TokenKind::Comma);
                true
            },
            _ => false,
        }
    }

    pub(crate) fn semi_colon(&mut self) -> Span {
        self.expect(TokenKind::SemiColon)
    }
}
