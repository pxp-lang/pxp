use pxp_ast::literals::{Literal, LiteralKind};
use pxp_diagnostics::Severity;
use pxp_token::TokenKind;

use crate::{state::State, ParserDiagnostic};

pub fn expect_literal(state: &mut State) -> Literal {
    let token = state.stream.current();
    let kind = match &token.kind {
        TokenKind::LiteralInteger => {
            state.stream.next();

            LiteralKind::Integer
        }
        TokenKind::LiteralFloat => {
            state.stream.next();

            LiteralKind::Float
        }
        TokenKind::LiteralSingleQuotedString | TokenKind::LiteralDoubleQuotedString => {
            state.stream.next();

            LiteralKind::String
        }
        _ => {
            state.diagnostic(
                ParserDiagnostic::ExpectedToken {
                    expected: vec![
                        TokenKind::LiteralInteger,
                        TokenKind::LiteralFloat,
                        TokenKind::LiteralSingleQuotedString,
                        TokenKind::LiteralDoubleQuotedString,
                    ],
                    found: *token,
                },
                Severity::Error,
                token.span,
            );

            return Literal::missing(token.span);
        }
    };

    Literal {
        kind,
        token: *token,
    }
}
