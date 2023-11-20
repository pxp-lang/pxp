use std::fmt::Display;

use crate::Expression;
use pxp_bytestring::ByteString;
use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Variable {
    SimpleVariable(SimpleVariable),
    VariableVariable(VariableVariable),
    BracedVariableVariable(BracedVariableVariable),
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct SimpleVariable {
    pub span: Span,
    pub name: ByteString,
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

impl Display for SimpleVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
