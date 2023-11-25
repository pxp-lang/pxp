use std::slice::Iter;

use crate::identifiers::SimpleIdentifier;

use crate::Expression;
use pxp_span::Span;
use pxp_syntax::comments::CommentGroup;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PositionalArgument {
    pub comments: CommentGroup,
    pub ellipsis: Option<Span>, // `...`
    pub value: Expression,      // `$var`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NamedArgument {
    pub comments: CommentGroup,
    pub name: SimpleIdentifier, // `foo`
    pub colon: Span,            // `:`
    pub ellipsis: Option<Span>, // `...`
    pub value: Expression,      // `$var`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Argument {
    Positional(PositionalArgument),
    Named(NamedArgument),
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ArgumentList {
    pub comments: CommentGroup,
    pub left_parenthesis: Span,   // `(`
    pub arguments: Vec<Argument>, // `$var`, `...$var`, `foo: $var`, `foo: ...$var`
    pub right_parenthesis: Span,  // `)`
}

impl ArgumentList {
    pub fn iter(&self) -> Iter<'_, Argument> {
        self.arguments.iter()
    }
}

impl IntoIterator for ArgumentList {
    type Item = Argument;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.arguments.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct SingleArgument {
    pub comments: CommentGroup,
    pub left_parenthesis: Span,  // `(`
    pub argument: Option<Argument>,      // `$var`
    pub right_parenthesis: Span, // `)`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ArgumentPlaceholder {
    pub comments: CommentGroup,
    pub left_parenthesis: Span,  // `(`
    pub ellipsis: Span,          // `...`
    pub right_parenthesis: Span, // `)`
}
