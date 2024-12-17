use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_token::TokenKind;

use crate::{Parser, ParserDiagnostic};

impl<'a> Parser<'a> {
    pub fn parse_literal(&mut self) -> Literal {
        let token = self.current().to_owned();
        let span = self.current_span();
        let kind = match self.current_kind() {
            TokenKind::LiteralInteger => self.next_but_first(|_| LiteralKind::Integer),
            TokenKind::LiteralFloat => self.next_but_first(|_| LiteralKind::Float),
            TokenKind::LiteralSingleQuotedString | TokenKind::LiteralDoubleQuotedString => {
                self.next_but_first(|_| LiteralKind::String)
            }
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![
                            TokenKind::LiteralInteger,
                            TokenKind::LiteralFloat,
                            TokenKind::LiteralSingleQuotedString,
                            TokenKind::LiteralDoubleQuotedString,
                        ],
                        found: token,
                    },
                    Severity::Error,
                    span,
                );

                return Literal::missing(self.id(), span);
            }
        };

        Literal {
            id: self.id(),
            span,
            kind,
            token,
        }
    }
}
