use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_token::TokenKind;

use crate::{state::State, ParserDiagnostic};

pub fn expect_literal(state: &mut State) -> Literal {
    let token = state.current();
    let kind = match &token.kind {
        TokenKind::LiteralInteger => {
            state.next();

            LiteralKind::Integer
        }
        TokenKind::LiteralFloat => {
            state.next();

            LiteralKind::Float
        }
        TokenKind::LiteralSingleQuotedString | TokenKind::LiteralDoubleQuotedString => {
            state.next();

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
                    found: token.clone(),
                },
                Severity::Error,
                token.span,
            );

            return Literal::missing(state.id(), token.span);
        }
    };

    Literal {
        id: state.id(),
        span: token.span,
        kind,
        token: token.clone(),
    }
}
