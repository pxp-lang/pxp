use crate::{ArrayExpression, ArrayItem, Expression, HasId};

impl ArrayExpression {
    pub fn is_list(&self) -> bool {
        self.items
            .iter()
            .all(|item| matches!(item, ArrayItem::Value(_)))
    }
}
