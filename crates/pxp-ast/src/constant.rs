use std::slice::Iter;

use crate::attributes::AttributeGroup;
use crate::comments::CommentGroup;
use crate::identifiers::SimpleIdentifier;
use crate::modifiers::ConstantModifierGroup;

use crate::Expression;
use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ConstantEntry {
    pub name: SimpleIdentifier, // `FOO`
    pub equals: Span,           // `=`
    pub value: Expression,      // `123`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ConstantStatement {
    pub comments: CommentGroup,
    pub r#const: Span,               // `const`
    pub entries: Vec<ConstantEntry>, // `FOO = 123`
    pub semicolon: Span,             // `;`
}

impl ConstantStatement {
    pub fn iter(&self) -> Iter<'_, ConstantEntry> {
        self.entries.iter()
    }
}

impl IntoIterator for ConstantStatement {
    type Item = ConstantEntry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ClassishConstant {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroup>,  // `#[Foo]`
    pub modifiers: ConstantModifierGroup, // `public`
    pub r#const: Span,                    // `const`

    pub entries: Vec<ConstantEntry>, // `FOO = 123`
    pub semicolon: Span,             // `;`
}

impl ClassishConstant {
    pub fn iter(&self) -> Iter<'_, ConstantEntry> {
        self.entries.iter()
    }
}

impl IntoIterator for ClassishConstant {
    type Item = ConstantEntry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}
