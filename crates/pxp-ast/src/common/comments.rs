use pxp_span::Span;
use pxp_token::Token;

#[derive(Debug, Clone)]
pub struct Comment {
    pub span: Span,
    pub token: Token
}

#[derive(Debug, Clone)]

pub struct CommentGroup {
    pub comments: Vec<Comment>,
}

impl CommentGroup {
    pub fn new(comments: Vec<Comment>) -> Self {
        Self { comments }
    }
}