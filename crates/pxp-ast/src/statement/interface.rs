use pxp_span::Span;

use crate::{SimpleIdentifier, ClassishConstant, ClassishMethod};

#[derive(Debug, Clone)]
pub struct InterfaceStatement {
    pub name: SimpleIdentifier,
    pub extends: Vec<SimpleIdentifier>,
    pub body: Vec<InterfaceMember>,
}

#[derive(Debug, Clone)]
pub struct InterfaceMember {
    pub kind: InterfaceMemberKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum InterfaceMemberKind {
    Constant(ClassishConstant),
    Method(ClassishMethod),
}