use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_token::TokenKind;

use crate::{state::State, Parser, ParserDiagnostic};

impl<'a> Parser<'a> {
    pub fn parse_literal(&mut self) -> Literal {
        let token = self.current();
        let kind = match &token.kind {
            TokenKind::LiteralInteger => {
                self.next();

                LiteralKind::Integer
            }
            TokenKind::LiteralFloat => {
                self.next();

                LiteralKind::Float
            }
            TokenKind::LiteralSingleQuotedString | TokenKind::LiteralDoubleQuotedString => {
                self.next();

                LiteralKind::String
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
                        found: token.clone(),
                    },
                    Severity::Error,
                    token.span,
                );

                return Literal::missing(self.state.id(), token.span);
            }
        };

        Literal {
            id: self.state.id(),
            span: token.span,
            kind,
            token: token.clone(),
        }
    }
}
