use pxp_span::Span;

use crate::{SimpleIdentifier, ClassishConstant, ClassishMethod, ClassishUse, ClassishProperty, AttributeGroup};

#[derive(Debug, Clone)]
pub struct TraitStatement {
    pub attributes: Vec<AttributeGroup>,
    pub name: SimpleIdentifier,
    pub body: Vec<TraitMember>,
}

#[derive(Debug, Clone)]
pub struct TraitMember {
    pub kind: TraitMemberKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum TraitMemberKind {
    Use(ClassishUse),
    Constant(ClassishConstant),
    Property(ClassishProperty),
    Method(ClassishMethod),
}