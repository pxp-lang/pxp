use crate::identifiers::SimpleIdentifier;

use crate::Block;
use pxp_span::Span;

use super::variables::SimpleVariable;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum CatchType {
    Identifier { identifier: SimpleIdentifier },
    Union { identifiers: Vec<SimpleIdentifier> },
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct TryStatement {
    pub start: Span,
    pub end: Span,
    pub body: Block,
    pub catches: Vec<CatchBlock>,
    pub finally: Option<FinallyBlock>,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct CatchBlock {
    pub start: Span,
    pub end: Span,
    pub types: CatchType,
    pub var: Option<SimpleVariable>,
    pub body: Block,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct FinallyBlock {
    pub start: Span,
    pub end: Span,
    pub body: Block,
}
