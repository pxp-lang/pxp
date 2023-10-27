use std::slice::Iter;

use pxp_span::Span;

use crate::{SimpleIdentifier, ArgumentList};

#[derive(Debug, Clone)]

pub struct Attribute {
    pub span: Span,
    pub name: SimpleIdentifier,
    pub arguments: Option<ArgumentList>,
}

#[derive(Debug, Clone)]

pub struct AttributeGroup {
    pub span: Span,
    pub members: Vec<Attribute>,
}

impl AttributeGroup {
    pub fn iter(&self) -> Iter<'_, Attribute> {
        self.members.iter()
    }
}

impl IntoIterator for AttributeGroup {
    type Item = Attribute;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.members.into_iter()
    }
}
