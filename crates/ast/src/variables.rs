use crate::{NodeId, SimpleVariable, Variable};
use pxp_bytestring::ByteString;
use pxp_span::{IsSpanned, Span};

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

impl IsSpanned for Variable {
    fn span(&self) -> Span {
        match self {
            Self::SimpleVariable(simple) => simple.span,
            Self::VariableVariable(dynamic) => dynamic.span,
            Self::BracedVariableVariable(dynamic) => dynamic.span,
        }
    }
}

impl Variable {
    pub fn to_simple(&self) -> &SimpleVariable {
        match self {
            Self::SimpleVariable(simple) => simple,
            _ => unreachable!(),
        }
    }

    pub fn is_simple(&self) -> bool {
        matches!(self, Self::SimpleVariable(_))
    }

    pub fn is_variable(&self) -> bool {
        matches!(
            self,
            Self::VariableVariable(_) | Self::BracedVariableVariable(_)
        )
    }
}
