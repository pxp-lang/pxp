use std::slice::Iter;

use pxp_span::{IsSpanned, Span};

use crate::{Comment, CommentGroup};

impl IsSpanned for CommentGroup {
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
