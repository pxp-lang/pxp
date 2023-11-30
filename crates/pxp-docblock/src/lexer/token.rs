use pxp_span::Span;
use pxp_symbol::Symbol;

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub symbol: Option<Symbol>,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, symbol: Option<Symbol>) -> Self {
        Self {
            kind,
            span,
            symbol,
        }
    }

    pub fn new_with_symbol(kind: TokenKind, span: Span, symbol: Symbol) -> Self {
        Self::new(kind, span, Some(symbol))
    }

    pub fn new_without_symbol(kind: TokenKind, span: Span) -> Self {
        Self::new(kind, span, None)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    QualifiedIdentifier,
    FullyQualifiedIdentifier,
    ThisVariable,
    Variable,
    Reference,
    Union,
    Intersection,
    Nullable,
    Negated,
    OpenParen,
    CloseParen,
    OpenAngle,
    CloseAngle,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
    Comma,
    Variadic,
    DoubleColon,
    DoubleArrow,
    Arrow,
    Equal,
    Colon,

    OpenDoc,
    CloseDoc,
    Tag,
    Eol,

    Float,
    Integer,
    SingleQuotedString,
    DoubleQuotedString,
    Asterisk,

    Other,
}