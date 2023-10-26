use crate::SimpleIdentifier;

#[derive(Debug, Clone)]
pub struct UseStatement {
    pub kind: UseKind,
    pub uses: Vec<Use>,
}

#[derive(Debug, Clone)]
pub enum UseKind {
    Normal,
    Function,
    Const,
}

#[derive(Debug, Clone)]
pub struct Use {
    pub name: SimpleIdentifier,
    pub alias: Option<SimpleIdentifier>,
}