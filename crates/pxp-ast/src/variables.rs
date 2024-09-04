use crate::{NodeId, SimpleVariable, Variable};
use pxp_bytestring::ByteString;
use pxp_span::{Span, Spanned};

impl SimpleVariable {
    pub fn missing(id: NodeId, span: Span) -> Self {
        Self {
            id,
            symbol: ByteString::empty(),
            stripped: ByteString::empty(),
            span,
        }
    }

    pub fn is_missing(&self) -> bool {
        self.symbol.is_empty()
    }
}

impl Spanned for Variable {
    fn span(&self) -> Span {
        match self {
            Self::SimpleVariable(simple) => simple.span,
            Self::VariableVariable(dynamic) => dynamic.span,
            Self::BracedVariableVariable(dynamic) => dynamic.span,
        }
    }
}
