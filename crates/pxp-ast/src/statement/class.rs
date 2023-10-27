use pxp_span::Span;
use pxp_type::Type;

use crate::{SimpleIdentifier, Constant, Expression, AttributeGroup, Visibility};

#[derive(Debug, Clone)]
pub struct ClassStatement {
    pub name: SimpleIdentifier,
    pub extends: Option<SimpleIdentifier>,
    pub implements: Vec<SimpleIdentifier>,
    pub body: Vec<ClassMember>,
}

#[derive(Debug, Clone)]
pub struct ClassMember {
    pub kind: ClassMemberKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ClassMemberKind {
    Constant(ClassishConstant),
    Use(ClassishUse),
    Property(ClassishProperty),
    Method(ClassishMethod),
}

#[derive(Debug, Clone)]
pub struct ClassishConstant {
    pub attributes: Vec<AttributeGroup>,
    pub constants: Vec<Constant>,
}

#[derive(Debug, Clone)]
pub struct ClassishUse {
    pub traits: Vec<SimpleIdentifier>,
    // FIXME: Add in trait adaptations here.
}

#[derive(Debug, Clone)]
pub struct ClassishProperty {
    pub attributes: Vec<AttributeGroup>,
    // FIXME: Add modifiers here.
    pub visibility: Visibility,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: Vec<SimpleIdentifier>,
    pub default: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct ClassishMethod {
    // FIXME: Add comments here.
    pub attributes: Vec<AttributeGroup>,
    pub visibility: Visibility,
    pub name: SimpleIdentifier,
    pub parameters: Vec<MethodParameter>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct MethodParameter {
    pub attributes: Vec<AttributeGroup>,
    pub name: SimpleIdentifier,
    pub r#type: Option<Type>,
    pub default: Option<Expression>,
    pub variadic: bool,
    pub by_reference: bool,
}