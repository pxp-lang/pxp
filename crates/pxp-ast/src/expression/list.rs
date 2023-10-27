use crate::Expression;

#[derive(Debug, Clone)]
pub struct ListExpression {
    pub items: Vec<ListEntry>,
}

#[derive(Debug, Clone)]
pub enum ListEntry {
    Skipped,
    Value(Expression),
    KeyValue(Expression, Expression),
}