use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::node::Node;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::ast::Block;

use super::variables::SimpleVariable;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum CatchType {
    Identifier { identifier: SimpleIdentifier },
    Union { identifiers: Vec<SimpleIdentifier> },
}

impl Node for CatchType {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            CatchType::Identifier { identifier } => vec![identifier],
            CatchType::Union { identifiers } => {
                identifiers.iter_mut().map(|i| i as &mut dyn Node).collect()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct TryStatement {
    pub start: Span,
    pub end: Span,
    pub body: Block,
    pub catches: Vec<CatchBlock>,
    pub finally: Option<FinallyBlock>,
}

impl Node for TryStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.body];
        for catch in &mut self.catches {
            children.push(catch);
        }
        if let Some(finally) = &mut self.finally {
            children.push(finally);
        }
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct CatchBlock {
    pub start: Span,
    pub end: Span,
    pub types: CatchType,
    pub var: Option<SimpleVariable>,
    pub body: Block,
}

impl Node for CatchBlock {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children = vec![&mut self.types as &mut dyn Node];
        if let Some(var) = &mut self.var {
            children.push(var as &mut dyn Node);
        }
        children.push(&mut self.body as &mut dyn Node);
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct FinallyBlock {
    pub start: Span,
    pub end: Span,
    pub body: Block,
}

impl Node for FinallyBlock {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.body as &mut dyn Node]
    }
}
