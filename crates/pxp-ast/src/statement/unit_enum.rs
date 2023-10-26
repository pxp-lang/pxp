use pxp_span::Span;

use crate::{SimpleIdentifier, ClassishMethod, ClassishConstant, ClassishUse};

#[derive(Debug, Clone)]
pub struct UnitEnumStatement {
    // FIXME: Add attributes here.
    pub name: SimpleIdentifier,
    pub implements: Vec<SimpleIdentifier>,
    pub body: Vec<UnitEnumMember>,
}

#[derive(Debug, Clone)]
pub struct UnitEnumMember {
    pub kind: UnitEnumMemberKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum UnitEnumMemberKind {
    Case(UnitEnumCase),
    Method(ClassishMethod),
    Constant(ClassishConstant),
    Use(ClassishUse),
}

#[derive(Debug, Clone)]
pub struct UnitEnumCase {
    // FIXME: Add attributes here.
    pub name: SimpleIdentifier,
}