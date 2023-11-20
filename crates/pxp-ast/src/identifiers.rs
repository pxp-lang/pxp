use std::fmt::Display;


use crate::Expression;
use pxp_bytestring::ByteString;
use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Identifier {
    SimpleIdentifier(SimpleIdentifier),
    DynamicIdentifier(DynamicIdentifier),
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct SimpleIdentifier {
    pub span: Span,
    pub value: ByteString,
}

impl Display for SimpleIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct DynamicIdentifier {
    pub start: Span,
    pub expr: Box<Expression>,
    pub end: Span,
}