use crate::SimpleIdentifier;

#[derive(Debug, Clone)]
pub struct LabelStatement {
    pub label: SimpleIdentifier,
}