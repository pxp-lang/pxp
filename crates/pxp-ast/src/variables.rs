use crate::Expression;
use pxp_span::Span;
use pxp_token::Token;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Variable {
    SimpleVariable(SimpleVariable),
    VariableVariable(VariableVariable),
    BracedVariableVariable(BracedVariableVariable),
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct SimpleVariable {
    pub token: Token,
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
