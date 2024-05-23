use crate::SimpleVariable;
use pxp_span::Span;
use pxp_symbol::Symbol;

impl SimpleVariable {
    pub fn missing(span: Span) -> Self {
        Self { symbol: Symbol::missing(), span }
    }
}
