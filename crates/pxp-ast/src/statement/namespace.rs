use pxp_span::Span;

use crate::{SimpleIdentifier, Block};

#[derive(Debug, Clone)]
pub struct NamespaceStatement {
    pub kind: NamespaceKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum NamespaceKind {
    Unbraced(UnbracedNamespace),
    Braced(BracedNamespace),
}

#[derive(Debug, Clone)]
pub struct UnbracedNamespace {
    pub name: SimpleIdentifier,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct BracedNamespace {
    pub name: Option<SimpleIdentifier>,
    pub body: Block,
}

impl BracedNamespace {
    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }
}