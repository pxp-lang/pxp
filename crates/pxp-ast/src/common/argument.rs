use pxp_span::Span;

use crate::{Expression, SimpleIdentifier};

#[derive(Debug, Clone)]
pub struct Argument {
    pub kind: ArgumentKind,
    pub span: Span
}

#[derive(Debug, Clone)]
pub enum ArgumentKind {
    Named(NamedArgument),
    Positional(PositionalArgument),
}

#[derive(Debug, Clone)]
pub struct NamedArgument {
    pub name: SimpleIdentifier,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct PositionalArgument {
    pub value: Expression,
    pub splat: bool,
}

#[derive(Debug, Clone)]
pub struct ArgumentList {
    pub arguments: Vec<Argument>,
    pub span: Span,
}