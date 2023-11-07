use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::byte_string::ByteString;
use crate::lexer::token::Span;
use crate::node::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Literal {
    String(LiteralString),
    Integer(LiteralInteger),
    Float(LiteralFloat),
}

impl Node for Literal {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            Literal::String(literal) => vec![literal],
            Literal::Integer(literal) => vec![literal],
            Literal::Float(literal) => vec![literal],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct LiteralString {
    pub value: ByteString,
    pub span: Span,
    pub kind: LiteralStringKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub enum LiteralStringKind {
    SingleQuoted,
    DoubleQuoted,
}

impl Node for LiteralString {
    //
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct LiteralInteger {
    pub value: ByteString,
    pub span: Span,
}

impl Node for LiteralInteger {
    //
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct LiteralFloat {
    pub value: ByteString,
    pub span: Span,
}

impl Node for LiteralFloat {
    //
}
