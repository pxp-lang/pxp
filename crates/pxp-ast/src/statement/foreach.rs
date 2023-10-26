use crate::{Block, Expression};

#[derive(Debug, Clone)]
pub struct ForeachStatement {
    pub iterable: Expression,
    pub key: Option<Expression>,
    pub value: Expression,
    pub body: Block,
    pub by_reference: bool,
}