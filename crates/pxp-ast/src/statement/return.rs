use crate::Expression;

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

impl ReturnStatement {
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
}