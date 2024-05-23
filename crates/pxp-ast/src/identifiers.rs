use pxp_span::Span;
use pxp_symbol::Symbol;

use crate::{Identifier, SimpleIdentifier};

impl Identifier {
    pub fn missing() -> Self {
        Self::SimpleIdentifier(SimpleIdentifier::new(Symbol::missing(), Span::default()))
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

impl SimpleIdentifier {
    pub fn new(symbol: Symbol, span: Span) -> Self {
        Self { symbol, span }
    }
}
