use crate::{Expression, NodeId};
use pxp_span::Span;
use pxp_symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Identifier {
    SimpleIdentifier(SimpleIdentifier),
    DynamicIdentifier(DynamicIdentifier),
}

impl Identifier {
    pub fn missing() -> Self {
        Self::SimpleIdentifier(SimpleIdentifier::new(0, Symbol::missing(), Span::default()))
    }

    pub fn is_simple(&self) -> bool {
        match self {
            Self::SimpleIdentifier(..) => true,
            Self::DynamicIdentifier(..) => false,
        }
    }

    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::SimpleIdentifier(..) => false,
            Self::DynamicIdentifier(..) => true,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct SimpleIdentifier {
    pub id: NodeId,
    pub symbol: Symbol,
    pub span: Span,
}

impl SimpleIdentifier {
    pub fn new(id: NodeId, symbol: Symbol, span: Span) -> Self {
        Self { id, symbol, span }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct DynamicIdentifier {
    pub span: Span,
    pub expr: Box<Expression>,
}
