use pxp_span::Span;

use crate::{SimpleIdentifier, ClassishMethod, ClassishConstant, ClassishUse, Expression, AttributeGroup};

#[derive(Debug, Clone)]
pub struct BackedEnumStatement {
    pub attributes: Vec<AttributeGroup>,
    pub name: SimpleIdentifier,
    pub backing_type: BackedEnumType,
    pub implements: Vec<SimpleIdentifier>,
    pub body: Vec<BackedEnumMember>,
}

#[derive(Debug, Clone)]
pub enum BackedEnumType {
    Int,
    String,
}

#[derive(Debug, Clone)]
pub struct BackedEnumMember {
    pub kind: BackedEnumMemberKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum BackedEnumMemberKind {
    Case(BackedEnumCase),
    Method(ClassishMethod),
    Constant(ClassishConstant),
    Use(ClassishUse),
}

#[derive(Debug, Clone)]
pub struct BackedEnumCase {
    pub attributes: Vec<AttributeGroup>,
    pub name: SimpleIdentifier,
    pub value: Expression,
}