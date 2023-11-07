use std::slice::Iter;

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::node::Node;
use crate::parser::ast::attributes::AttributeGroup;
use crate::parser::ast::comments::CommentGroup;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::ast::modifiers::ConstantModifierGroup;
use crate::parser::ast::Expression;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ConstantEntry {
    pub name: SimpleIdentifier, // `FOO`
    pub equals: Span,           // `=`
    pub value: Expression,      // `123`
}

impl Node for ConstantEntry {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.name, &mut self.value]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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

impl Node for ConstantStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.entries
            .iter_mut()
            .map(|e| e as &mut dyn Node)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ClassishConstant {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroup>,  // `#[Foo]`
    pub modifiers: ConstantModifierGroup, // `public`
    pub r#const: Span,                    // `const`
    #[serde(flatten)]
    pub entries: Vec<ConstantEntry>, // `FOO = 123`
    pub semicolon: Span,                  // `;`
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

impl Node for ClassishConstant {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.entries
            .iter_mut()
            .map(|e| e as &mut dyn Node)
            .collect()
    }
}
