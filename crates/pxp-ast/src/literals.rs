use pxp_span::Span;
use pxp_token::Token;

use crate::{Literal, LiteralKind};

impl Literal {
    pub fn new(kind: LiteralKind, token: Token, span: Span) -> Literal {
        Literal { kind, token, span }
    }

    pub fn missing(span: Span) -> Literal {
        Literal {
            kind: LiteralKind::Missing,
            token: Token::missing(span),
            span,
        }
    }
}
