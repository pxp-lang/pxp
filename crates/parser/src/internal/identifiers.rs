use pxp_ast::SimpleIdentifier;
use pxp_token::TokenKind;

use crate::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_unqualified_identifier(&mut self) -> SimpleIdentifier {
        if self.current_kind() != TokenKind::Identifier {
            self.expected_token(TokenKind::Identifier);

            return SimpleIdentifier::missing(self.id(), self.current_span());
        }

        self.next_but_first(|parser| SimpleIdentifier {
            id: parser.id(),
            symbol: parser.current_symbol().to_bytestring(),
            span: parser.current_span(),
        })
    }

    pub(crate) fn parse_unqualified_or_qualified_identifier(&mut self) -> SimpleIdentifier {
        if !matches!(self.current_kind(), TokenKind::Identifier | TokenKind::QualifiedIdentifier) {
            self.expected_any_of_tokens(&[TokenKind::Identifier, TokenKind::QualifiedIdentifier]);

            return SimpleIdentifier::missing(self.id(), self.current_span());
        }

        self.next_but_first(|parser| SimpleIdentifier {
            id: parser.id(),
            symbol: parser.current_symbol().to_bytestring(),
            span: parser.current_span(),
        })
    }
}
