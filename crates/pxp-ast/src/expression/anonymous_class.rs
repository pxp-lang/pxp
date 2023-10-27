use crate::{SimpleIdentifier, ClassMember, AttributeGroup};

#[derive(Debug, Clone)]
pub struct AnonymousClassExpression {
    pub attributes: Vec<AttributeGroup>,
    pub extends: Option<SimpleIdentifier>,
    pub implements: Vec<SimpleIdentifier>,
    pub body: Vec<ClassMember>,
}