use pxp_span::Span;
use pxp_token::Token;

use crate::{Literal, LiteralKind, NodeId};

impl Literal {
    pub fn new(id: NodeId, kind: LiteralKind, token: Token, span: Span) -> Literal {
        Literal { id, kind, token, span }
    }

    pub fn missing(id: NodeId, span: Span) -> Literal {
        Literal {
            id,
            kind: LiteralKind::Missing,
            token: Token::missing(span),
            span,
        }
    }
}
