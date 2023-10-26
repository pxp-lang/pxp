use crate::SimpleIdentifier;

#[derive(Debug, Clone)]
pub struct GotoStatement {
    pub label: SimpleIdentifier,
}