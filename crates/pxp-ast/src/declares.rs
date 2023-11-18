use crate::identifiers::SimpleIdentifier;
use crate::literals::Literal;
use crate::node::Node;
use crate::Expression;
use crate::Statement;

use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct DeclareEntry {
    pub key: SimpleIdentifier, // `strict_types`
    pub equals: Span,          // `=`
    pub value: Literal,        // `1`
}

impl Node for DeclareEntry {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.key, &mut self.value]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct DeclareEntryGroup {
    pub left_parenthesis: Span,     // `(`
    pub right_parenthesis: Span,    // `)`
    pub entries: Vec<DeclareEntry>, // `strict_types = 1`
}

impl Node for DeclareEntryGroup {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.entries
            .iter_mut()
            .map(|e| e as &mut dyn Node)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum DeclareBody {
    // declaration is terminated with `;`
    Noop {
        semicolon: Span, // `;`
    },
    // declaration is followed by a `{` and terminated with `}` after multiple statements.
    Braced {
        left_brace: Span,           // `{`
        statements: Vec<Statement>, // `*statements*`
        right_brace: Span,          // `}`
    },
    // declaration is terminated with `;` after a single expression.
    Expression {
        expression: Expression, // `*expression*`
        semicolon: Span,        // `;`
    },
    // declaration is followed by a `:` and terminated with `enddeclare` and `;` after multiple statements.
    Block {
        colon: Span,                // `:`
        statements: Vec<Statement>, // `*statements*`
        end: (Span, Span),          // `enddeclare` + `;`
    },
}

impl Node for DeclareBody {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            DeclareBody::Noop { .. } => vec![],
            DeclareBody::Braced { statements, .. } => {
                statements.iter_mut().map(|s| s as &mut dyn Node).collect()
            }
            DeclareBody::Expression { expression, .. } => vec![expression],
            DeclareBody::Block { statements, .. } => {
                statements.iter_mut().map(|s| s as &mut dyn Node).collect()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct DeclareStatement {
    pub declare: Span,              // `declare`
    pub entries: DeclareEntryGroup, // `(strict_types = 1)`
    pub body: DeclareBody,          // `;`
}

impl Node for DeclareStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.entries, &mut self.body]
    }
}
