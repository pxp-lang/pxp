use crate::{SimpleIdentifier, UseKind};

#[derive(Debug, Clone)]
pub struct GroupUseStatement {
    pub prefix: SimpleIdentifier,
    pub kind: UseKind,
    pub uses: Vec<GroupUse>,
}

#[derive(Debug, Clone)]
pub struct GroupUse {
    pub name: SimpleIdentifier,
    pub alias: Option<SimpleIdentifier>,
    pub kind: Option<UseKind>,
}