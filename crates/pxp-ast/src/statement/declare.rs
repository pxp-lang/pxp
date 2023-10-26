use crate::{Literal, SimpleIdentifier, Block, Expression};

#[derive(Debug, Clone)]
pub struct DeclareStatement {
    pub declares: Vec<Declare>,
    pub kind: DeclareKind,
}

#[derive(Debug, Clone)]
pub struct Declare {
    pub name: SimpleIdentifier,
    pub value: Literal,
}

#[derive(Debug, Clone)]
pub enum DeclareKind {
    Noop,
    Block(Block),
    Expression(Expression),
}