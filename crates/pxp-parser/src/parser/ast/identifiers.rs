use std::fmt::Display;

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::byte_string::ByteString;
use crate::lexer::token::Span;
use crate::node::Node;
use crate::parser::ast::Expression;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Identifier {
    SimpleIdentifier(SimpleIdentifier),
    DynamicIdentifier(DynamicIdentifier),
}

impl Node for Identifier {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            Identifier::SimpleIdentifier(identifier) => identifier.children(),
            Identifier::DynamicIdentifier(identifier) => identifier.children(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct SimpleIdentifier {
    pub span: Span,
    pub value: ByteString,
}

impl Node for SimpleIdentifier {
    //
}

impl Display for SimpleIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct DynamicIdentifier {
    pub start: Span,
    pub expr: Box<Expression>,
    pub end: Span,
}

impl Node for DynamicIdentifier {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.expr.as_mut()]
    }
}
