use pxp_span::Span;
use pxp_token::Token;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Literal {
    pub kind: LiteralKind,
    pub token: Token,
}

impl Literal {
    pub fn new(kind: LiteralKind, token: Token) -> Literal {
        Literal { kind, token }
    }

    pub fn missing(span: Span) -> Literal {
        Literal {
            kind: LiteralKind::Missing,
            token: Token::missing(span),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LiteralKind {
    Integer,
    Float,
    String,
    Missing,
}
