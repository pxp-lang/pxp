use crate::{Expression, StatementOrBlock};

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: StatementOrBlock,
    pub else_ifs: Vec<ElseIf>,
    pub r#else: Else,
}

#[derive(Debug, Clone)]
pub struct ElseIf {
    pub condition: Expression,
    pub body: StatementOrBlock,
}

#[derive(Debug, Clone)]
pub struct Else {
    pub body: StatementOrBlock,
}