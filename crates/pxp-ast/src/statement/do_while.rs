use crate::{Block, Expression};

#[derive(Debug, Clone)]
pub struct DoWhileStatement {
    pub body: Block,
    pub condition: Expression,
}