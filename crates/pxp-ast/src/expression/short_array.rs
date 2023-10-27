use crate::{CommaSeparated, Expression};

#[derive(Debug, Clone)]
pub struct ShortArrayExpression {
    pub items: CommaSeparated<ArrayItem>,
}

#[derive(Debug, Clone)]
pub enum ArrayItem {
    Skipped,
    Value(Expression),
    Reference(Expression),
    Spread(Expression),
    KeyValue(Expression, Expression),
    ReferenceKeyValue(Expression, Expression)
}