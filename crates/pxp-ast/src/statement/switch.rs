use crate::{Expression, Block};

#[derive(Debug, Clone)]
pub struct SwitchStatement {
    pub subject: Expression,
    pub cases: Vec<SwitchCase>,
}

#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub condition: Option<Expression>,
    pub body: Block,
}

impl SwitchCase {
    pub fn is_default(&self) -> bool {
        self.condition.is_none()
    }
}