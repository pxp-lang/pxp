use pxp_span::Span;
use pxp_token::Token;

use crate::Expression;

#[derive(Debug, Clone)]
pub struct Variable {
    pub kind: VariableKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum VariableKind {
    Simple(SimpleVariable),
    Variable(VariableVariable),
    Braced(BracedVariable),
}

#[derive(Debug, Clone)]
pub struct SimpleVariable {
    pub name: Token,
    pub span: Span
}

#[derive(Debug, Clone)]
pub struct VariableVariable {
    pub name: Box<Variable>,
    pub span: Span
}

#[derive(Debug, Clone)]
pub struct BracedVariable {
    pub name: Box<Expression>,
    pub span: Span
}