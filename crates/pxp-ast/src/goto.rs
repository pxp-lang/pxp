use crate::comments::CommentGroup;
use crate::identifiers::SimpleIdentifier;

use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct LabelStatement {
    pub comments: CommentGroup,
    pub label: SimpleIdentifier, // `foo`
    pub colon: Span,             // `:`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct GotoStatement {
    pub comments: CommentGroup,
    pub keyword: Span,           // `goto`
    pub label: SimpleIdentifier, // `foo`
    pub semicolon: Span,         // `;`
}
