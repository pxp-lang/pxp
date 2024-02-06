use crate::{Expression, NodeId};
use pxp_span::Span;
use pxp_symbol::Symbol;
use pxp_syntax::identifier::IdentifierQualification;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Identifier {
    SimpleIdentifier(SimpleIdentifier),
    DynamicIdentifier(DynamicIdentifier),
}

impl Identifier {
    pub fn missing() -> Self {
        Self::SimpleIdentifier(SimpleIdentifier::new(0, 0, IdentifierQualification::Unqualified, Span::default()))
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
    pub qualification: IdentifierQualification,
    pub span: Span,
}

impl SimpleIdentifier {
    pub fn new(id: NodeId, symbol: Symbol, qualification: IdentifierQualification, span: Span) -> Self {
        Self { id, symbol, qualification, span }
    }

    pub fn is_fully_qualified(&self) -> bool {
        self.qualification.is_fully_qualified()
    }

    pub fn is_qualified(&self) -> bool {
        self.qualification.is_qualified()
    }

    pub fn is_unqualified(&self) -> bool {
        self.qualification.is_unqualified()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct DynamicIdentifier {
    pub span: Span,
    pub expr: Box<Expression>,
}
