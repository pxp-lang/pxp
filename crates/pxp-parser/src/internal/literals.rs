use pxp_ast::literals::{Literal, LiteralKind};
use pxp_token::TokenKind;

use crate::state::State;

pub fn expect_literal(state: &mut State) -> Literal {
    let token = state.stream.current();
    let kind = match &token.kind {
        TokenKind::LiteralInteger => {
            state.stream.next();

            LiteralKind::Integer
        },
        TokenKind::LiteralFloat => {
            state.stream.next();

            LiteralKind::Float
        }
        TokenKind::LiteralSingleQuotedString | TokenKind::LiteralDoubleQuotedString => {
            state.stream.next();

            LiteralKind::String
        }
        _ => {
            todo!("tolerant handling of missing literals (return Literal::missing() or something)")
        }
    };

    Literal { kind, token: *token }
}