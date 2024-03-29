use std::slice::Iter;

use crate::arguments::ArgumentList;
use crate::identifiers::SimpleIdentifier;
use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Attribute {
    pub span: Span,
    pub name: SimpleIdentifier,
    pub arguments: Option<ArgumentList>,
}

#[derive(Debug, PartialEq, Eq, Clone)]

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
