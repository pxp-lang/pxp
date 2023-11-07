use std::slice::Iter;

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::node::Node;
use crate::parser::ast::comments::CommentGroup;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::ast::Expression;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct PositionalArgument {
    pub comments: CommentGroup,
    pub ellipsis: Option<Span>, // `...`
    pub value: Expression,      // `$var`
}

impl Node for PositionalArgument {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.value]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct NamedArgument {
    pub comments: CommentGroup,
    pub name: SimpleIdentifier, // `foo`
    pub colon: Span,            // `:`
    pub ellipsis: Option<Span>, // `...`
    pub value: Expression,      // `$var`
}

impl Node for NamedArgument {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.name, &mut self.value]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Argument {
    Positional(PositionalArgument),
    Named(NamedArgument),
}

impl Node for Argument {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            Argument::Positional(argument) => vec![argument],
            Argument::Named(argument) => vec![argument],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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

impl Node for ArgumentList {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.arguments
            .iter_mut()
            .map(|a| a as &mut dyn Node)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct SingleArgument {
    pub comments: CommentGroup,
    pub left_parenthesis: Span,  // `(`
    pub argument: Argument,      // `$var`
    pub right_parenthesis: Span, // `)`
}

impl Node for SingleArgument {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.argument]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ArgumentPlaceholder {
    pub comments: CommentGroup,
    pub left_parenthesis: Span,  // `(`
    pub ellipsis: Span,          // `...`
    pub right_parenthesis: Span, // `)`
}
