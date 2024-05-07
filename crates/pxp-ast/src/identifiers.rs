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
        Self::SimpleIdentifier(SimpleIdentifier::new(0, Symbol(0), Vec::from([Symbol(0)]), Span::default()))
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
    pub parts: Vec<Symbol>,
    pub span: Span,
}

impl SimpleIdentifier {
    pub fn new(id: NodeId, symbol: Symbol, parts: Vec<Symbol>, span: Span) -> Self {
        Self { id, symbol, parts, span }
    }

    pub fn get_first_part(&self) -> Symbol {
        self.parts.first().copied().unwrap_or(self.symbol)
    }

    pub fn get_last_part(&self) -> Symbol {
        self.parts.last().copied().unwrap_or(self.symbol)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct DynamicIdentifier {
    pub span: Span,
    pub expr: Box<Expression>,
}
