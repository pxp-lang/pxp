use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::node::Node;
use crate::parser::ast::Ending;
use crate::parser::ast::Expression;
use crate::parser::ast::Statement;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct IfStatement {
    pub r#if: Span,              // `if`
    pub left_parenthesis: Span,  // `(`
    pub condition: Expression,   // *expression*
    pub right_parenthesis: Span, // `)`
    pub body: IfStatementBody,   // `{ ... }`
}

impl Node for IfStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.condition, &mut self.body]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum IfStatementBody {
    Statement {
        statement: Box<Statement>,       // `*statement*`
        elseifs: Vec<IfStatementElseIf>, // `elseif (*expression*) *statement*`
        r#else: Option<IfStatementElse>, // `else *statement*`
    },
    Block {
        colon: Span,                          // `:`
        statements: Vec<Statement>,           // `*statements*`
        elseifs: Vec<IfStatementElseIfBlock>, // `elseif (*expression*): *statements*`
        r#else: Option<IfStatementElseBlock>, // `else: *statements*`
        endif: Span,                          // `endif`
        ending: Ending,                       // `;` or `?>`
    },
}

impl Node for IfStatementBody {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            IfStatementBody::Statement {
                statement,
                elseifs,
                r#else,
            } => {
                let mut children: Vec<&mut dyn Node> = vec![statement.as_mut()];
                children.extend(
                    elseifs
                        .iter_mut()
                        .map(|elseif| elseif as &mut dyn Node)
                        .collect::<Vec<&mut dyn Node>>(),
                );
                if let Some(r#else) = r#else {
                    children.push(r#else as &mut dyn Node);
                }
                children
            }
            IfStatementBody::Block {
                statements,
                elseifs,
                r#else,
                ..
            } => {
                let mut children: Vec<&mut dyn Node> = vec![];
                children.extend(
                    statements
                        .iter_mut()
                        .map(|statement| statement as &mut dyn Node),
                );
                children.extend(elseifs.iter_mut().map(|elseif| elseif as &mut dyn Node));
                if let Some(r#else) = r#else {
                    children.push(r#else as &mut dyn Node);
                }
                children
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct IfStatementElseIf {
    pub elseif: Span,              // `elseif`
    pub left_parenthesis: Span,    // `(`
    pub condition: Expression,     // `( *expression* )`
    pub right_parenthesis: Span,   // `)`
    pub statement: Box<Statement>, // `*statement*`
}

impl Node for IfStatementElseIf {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.condition, self.statement.as_mut()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct IfStatementElse {
    pub r#else: Span,              // `else`
    pub statement: Box<Statement>, // `*statement*`
}

impl Node for IfStatementElse {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.statement.as_mut()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct IfStatementElseIfBlock {
    pub elseif: Span,               // `elseif`
    pub left_parenthesis: Span,     // `(`
    pub condition: Expression,      // `( *expression* )`
    pub right_parenthesis: Span,    // `)`
    pub colon: Span,                // `:`
    pub statements: Vec<Statement>, // `*statements*`
}

impl Node for IfStatementElseIfBlock {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.condition];
        children.extend(
            self.statements
                .iter_mut()
                .map(|statement| statement as &mut dyn Node),
        );
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct IfStatementElseBlock {
    pub r#else: Span,               // `else`
    pub colon: Span,                // `:`
    pub statements: Vec<Statement>, // `*statements*`
}

impl Node for IfStatementElseBlock {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.statements
            .iter_mut()
            .map(|statement| statement as &mut dyn Node)
            .collect()
    }
}
