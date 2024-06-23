use std::fmt::{Display, Formatter};

mod generated;

pub use generated::*;
use pxp_span::Span;
use pxp_syntax::comments::CommentGroup;
use pxp_token::{Token, TokenKind};

pub mod data_type;
pub mod identifiers;
pub mod literals;
pub mod modifiers;
pub mod name;
pub mod operators;
pub mod properties;
pub mod utils;
pub mod variables;

impl Display for UseKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UseKind::Normal => write!(f, "use"),
            UseKind::Function => write!(f, "use function"),
            UseKind::Const => write!(f, "use const"),
        }
    }
}

impl Ending {
    pub fn span(&self) -> Span {
        match self {
            Ending::Semicolon(span) => *span,
            Ending::CloseTag(span) => *span,
            Ending::Missing(span) => *span,
        }
    }
}

impl Statement {
    pub fn new(kind: StatementKind, span: Span, comments: CommentGroup) -> Self {
        Self {
            span,
            kind,
            comments,
        }
    }
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Span, comments: CommentGroup) -> Self {
        Self {
            span,
            kind,
            comments,
        }
    }

    pub fn missing(span: Span) -> Self {
        Self::new(ExpressionKind::Missing(span), span, CommentGroup::default())
    }

    pub fn noop(span: Span) -> Self {
        Self::new(ExpressionKind::Noop(span), span, CommentGroup::default())
    }
}

impl From<Token> for CastKind {
    fn from(token: Token) -> Self {
        match token.kind {
            TokenKind::StringCast | TokenKind::BinaryCast => Self::String(token.span),
            TokenKind::ObjectCast => Self::Object(token.span),
            TokenKind::BoolCast | TokenKind::BooleanCast => Self::Bool(token.span),
            TokenKind::IntCast | TokenKind::IntegerCast => Self::Int(token.span),
            TokenKind::FloatCast | TokenKind::DoubleCast | TokenKind::RealCast => {
                Self::Float(token.span)
            }
            TokenKind::UnsetCast => Self::Unset(token.span),
            TokenKind::ArrayCast => Self::Array(token.span),
            _ => unreachable!(),
        }
    }
}

impl From<&TokenKind> for CastKind {
    fn from(kind: &TokenKind) -> Self {
        kind.into()
    }
}

impl From<Token> for SpecialNameKind {
    fn from(token: Token) -> Self {
        match token.kind {
            TokenKind::Self_ => Self::Self_(token.span),
            TokenKind::Parent => Self::Parent(token.span),
            TokenKind::Static => Self::Static(token.span),
            _ => unreachable!(),
        }
    }
}
