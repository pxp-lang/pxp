
use pxp_bytestring::ByteString;
use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Literal {
    String(LiteralString),
    Integer(LiteralInteger),
    Float(LiteralFloat),
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct LiteralString {
    pub value: ByteString,
    pub span: Span,
    pub kind: LiteralStringKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LiteralStringKind {
    SingleQuoted,
    DoubleQuoted,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct LiteralInteger {
    pub value: ByteString,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct LiteralFloat {
    pub value: ByteString,
    pub span: Span,
}
