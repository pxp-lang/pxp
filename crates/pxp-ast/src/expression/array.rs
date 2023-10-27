use crate::{CommaSeparated, ArrayItem};

#[derive(Debug, Clone)]
pub struct ArrayExpression {
    pub items: CommaSeparated<ArrayItem>,
}