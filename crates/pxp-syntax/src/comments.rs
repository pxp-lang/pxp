use std::slice::Iter;

use pxp_span::Span;
use pxp_symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CommentFormat {
    SingleLine,
    MultiLine,
    HashMark,
    Document,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Comment {
    pub span: Span,
    pub format: CommentFormat,
    pub content: Symbol,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]

pub struct CommentGroup {
    pub comments: Vec<Comment>,
}

impl CommentGroup {
    pub fn iter(&self) -> Iter<'_, Comment> {
        self.comments.iter()
    }
}

impl IntoIterator for CommentGroup {
    type Item = Comment;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.comments.into_iter()
    }
}
