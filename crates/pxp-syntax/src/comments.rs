use std::slice::Iter;

use pxp_bytestring::ByteString;
use pxp_span::{Span, Spanned};


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
    pub content: ByteString,
}

impl Spanned for Comment {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]

pub struct CommentGroup {
    pub comments: Vec<Comment>,
}

impl Spanned for CommentGroup {
    fn span(&self) -> Span {
        self.comments.span()
    }
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
