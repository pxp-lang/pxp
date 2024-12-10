use pxp_span::Span;
use pxp_token::{OwnedToken, Token};

use crate::{Literal, LiteralKind, NodeId};

impl Literal {
    pub fn new(id: NodeId, kind: LiteralKind, token: OwnedToken, span: Span) -> Literal {
        Literal {
            id,
            kind,
            token,
            span,
        }
    }

    pub fn missing(id: NodeId, span: Span) -> Literal {
        Literal {
            id,
            kind: LiteralKind::Missing,
            token: Token::missing(span).to_owned(),
            span,
        }
    }
}
