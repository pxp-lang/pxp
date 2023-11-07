use std::slice::Iter;

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::byte_string::ByteString;
use crate::lexer::token::Span;
use crate::node::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub enum CommentFormat {
    SingleLine,
    MultiLine,
    HashMark,
    Document,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct Comment {
    pub span: Span,
    pub format: CommentFormat,
    pub content: ByteString,
}

impl Node for Comment {}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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
