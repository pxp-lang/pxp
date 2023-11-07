use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::node::Node;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::ast::Statement;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct UnbracedNamespace {
    pub start: Span,                // `namespace`
    pub name: SimpleIdentifier,     // `Foo`
    pub end: Span,                  // `;`
    pub statements: Vec<Statement>, // `*statements*`
}

impl Node for UnbracedNamespace {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children = vec![&mut self.name as &mut dyn Node];
        children.extend(
            self.statements
                .iter_mut()
                .map(|s| s as &mut dyn Node)
                .collect::<Vec<&mut dyn Node>>(),
        );
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct BracedNamespace {
    pub namespace: Span,                // `namespace`
    pub name: Option<SimpleIdentifier>, // `Foo`
    pub body: BracedNamespaceBody,      // `{ *statements* }`
}

impl Node for BracedNamespace {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![];
        if let Some(name) = &mut self.name {
            children.push(name);
        }
        children.push(&mut self.body);
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct BracedNamespaceBody {
    pub start: Span,                // `{`
    pub end: Span,                  // `}`
    pub statements: Vec<Statement>, // `*statements*`
}

impl Node for BracedNamespaceBody {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.statements
            .iter_mut()
            .map(|s| s as &mut dyn Node)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum NamespaceStatement {
    Unbraced(UnbracedNamespace), // `namespace Foo; *statements*`
    Braced(BracedNamespace),     // `namespace Foo { *statements* }`
}

impl Node for NamespaceStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            NamespaceStatement::Unbraced(namespace) => vec![namespace],
            NamespaceStatement::Braced(namespace) => vec![namespace],
        }
    }
}
