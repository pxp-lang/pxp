use std::fmt::{Display, Formatter};

mod generated;

pub use generated::*;
use pxp_span::Span;
use pxp_syntax::comments::CommentGroup;
use pxp_token::TokenKind;

pub mod spanned;
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
        Self::new(ExpressionKind::Missing, span, CommentGroup::default())
    }

    pub fn noop(span: Span) -> Self {
        Self::new(ExpressionKind::Noop, span, CommentGroup::default())
    }
}

impl From<TokenKind> for CastKind {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::StringCast | TokenKind::BinaryCast => Self::String,
            TokenKind::ObjectCast => Self::Object,
            TokenKind::BoolCast | TokenKind::BooleanCast => Self::Bool,
            TokenKind::IntCast | TokenKind::IntegerCast => Self::Int,
            TokenKind::FloatCast | TokenKind::DoubleCast | TokenKind::RealCast => Self::Float,
            TokenKind::UnsetCast => Self::Unset,
            TokenKind::ArrayCast => Self::Array,
            _ => unreachable!(),
        }
    }
}

impl From<&TokenKind> for CastKind {
    fn from(kind: &TokenKind) -> Self {
        kind.into()
    }
}

impl From<TokenKind> for SpecialNameKind {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Self_ => Self::Self_,
            TokenKind::Parent => Self::Parent,
            TokenKind::Static => Self::Static,
            _ => unreachable!(),
        }
    }
}
