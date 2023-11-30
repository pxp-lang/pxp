pub(crate) mod tag;
pub(crate) mod text;
pub(crate) mod template;
pub(crate) mod const_expr;

use pxp_span::Span;

use self::{tag::TagNode, text::TextNode};

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Tag(TagNode),
    Text(TextNode),
}