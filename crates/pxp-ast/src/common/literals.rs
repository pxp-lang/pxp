use pxp_span::Span;
use pxp_token::Token;

#[derive(Debug, Clone)]
pub struct Literal {
    pub kind: LiteralKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum LiteralKind {
    Integer(LiteralInteger),
    Float(LiteralFloat),
    String(LiteralString),
}

#[derive(Debug, Clone)]
pub struct LiteralInteger {
    pub value: Token,
    pub span: Span
}

#[derive(Debug, Clone)]
pub struct LiteralFloat {
    pub value: Token,
    pub span: Span
}

#[derive(Debug, Clone)]
pub struct LiteralString {
    pub value: Token,
    pub span: Span,
}