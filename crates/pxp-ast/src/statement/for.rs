use crate::{Block, CommaSeparated, Expression};

#[derive(Debug, Clone)]
pub struct ForStatement {
    pub initializations: CommaSeparated<Expression>,
    pub conditions: CommaSeparated<Expression>,
    pub iterations: CommaSeparated<Expression>,
    pub body: Block,
}