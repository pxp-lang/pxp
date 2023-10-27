use crate::{SimpleIdentifier, ClassMember};

#[derive(Debug, Clone)]
pub struct AnonymousClassExpression {
    // FIXME: Add attributes here.
    pub extends: Option<SimpleIdentifier>,
    pub implements: Vec<SimpleIdentifier>,
    pub body: Vec<ClassMember>,
}