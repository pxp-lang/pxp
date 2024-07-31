use pxp_span::{Span, Spanned};
use pxp_symbol::Symbol;

use crate::{Identifier, NodeId, SimpleIdentifier};

impl Identifier {
    pub fn missing(id: NodeId, span: Span) -> Self {
        Self::SimpleIdentifier(SimpleIdentifier::new(id, Symbol::missing(), span))
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

impl Spanned for Identifier {
    fn span(&self) -> Span {
        match self {
            Self::SimpleIdentifier(simple) => simple.span,
            Self::DynamicIdentifier(dynamic) => dynamic.span,
        }
    }
}

impl SimpleIdentifier {
    pub fn new(id: NodeId, symbol: Symbol, span: Span) -> Self {
        Self { id, symbol, span }
    }
}
