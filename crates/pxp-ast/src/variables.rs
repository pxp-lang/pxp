use crate::Expression;
use pxp_span::Span;
use pxp_symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Variable {
    SimpleVariable(SimpleVariable),
    VariableVariable(VariableVariable),
    BracedVariableVariable(BracedVariableVariable),
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct SimpleVariable {
    pub symbol: Symbol,
    pub span: Span,
}

impl SimpleVariable {
    pub fn missing(span: Span) -> Self {
        Self { symbol: Symbol(0), span }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct VariableVariable {
    pub span: Span,
    pub variable: Box<Variable>,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct BracedVariableVariable {
    pub start: Span,
    pub variable: Box<Expression>,
    pub end: Span,
}
