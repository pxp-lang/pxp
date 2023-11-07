use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::node::Node;
use crate::parser::ast::comments::CommentGroup;
use crate::parser::ast::identifiers::SimpleIdentifier;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct LabelStatement {
    pub comments: CommentGroup,
    pub label: SimpleIdentifier, // `foo`
    pub colon: Span,             // `:`
}

impl Node for LabelStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.label]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct GotoStatement {
    pub comments: CommentGroup,
    pub keyword: Span,           // `goto`
    pub label: SimpleIdentifier, // `foo`
    pub semicolon: Span,         // `;`
}

impl Node for GotoStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.label]
    }
}
