use std::slice::Iter;

use pxp_span::Span;
use crate::parser::ast::arguments::ArgumentList;
use crate::parser::ast::identifiers::SimpleIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Attribute {
    pub start: Span,
    pub end: Span,
    pub name: SimpleIdentifier,
    pub arguments: Option<ArgumentList>,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct AttributeGroup {
    pub start: Span,
    pub end: Span,
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
