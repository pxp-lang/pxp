use pxp_parser::{
    lexer::token::Span,
    parser::ast::{
        arguments::ArgumentList,
        comments::CommentGroup,
        functions::{Closure, ClosureUse, ClosureUseVariable, FunctionBody, FunctionParameterList},
        utils::CommaSeparated,
        DefaultMatchArm, Expression, MatchArmBody,
    },
    traverser::Visitor,
};

use crate::visitors::VariableFinderVisitor;

use super::Transpiler;

pub struct MultiLineMatchTranspiler;

impl MultiLineMatchTranspiler {
    fn maybe_transpile_match_arm_body(&self, body: &mut MatchArmBody) {
        match body {
            MatchArmBody::Block {
                left_brace,
                ref statements,
                right_brace,
            } => {
                let mut variable_finder = VariableFinderVisitor::default();
                variable_finder.visit_node(body).unwrap();
                let mut variables = Vec::new();
                for variable in variable_finder.variables() {
                    variables.push(variable.clone());
                }
                let closure = Closure {
                    comments: CommentGroup { comments: vec![] },
                    attributes: vec![],
                    r#static: None,
                    function: Span::default(),
                    ampersand: None,
                    parameters: FunctionParameterList {
                        comments: CommentGroup { comments: vec![] },
                        left_parenthesis: Span::default(),
                        parameters: CommaSeparated {
                            inner: vec![],
                            commas: vec![],
                        },
                        right_parenthesis: Span::default(),
                    },
                    uses: if variables.is_empty() {
                        None
                    } else {
                        Some(ClosureUse {
                            comments: CommentGroup { comments: vec![] },
                            r#use: Span::default(),
                            left_parenthesis: Span::default(),
                            variables: CommaSeparated {
                                inner: variables
                                    .into_iter()
                                    .map(|v| ClosureUseVariable {
                                        comments: CommentGroup { comments: vec![] },
                                        ampersand: Some(Span::default()),
                                        variable: v,
                                    })
                                    .collect(),
                                commas: vec![],
                            },
                            right_parenthesis: Span::default(),
                        })
                    },
                    return_type: None,
                    body: FunctionBody {
                        comments: CommentGroup { comments: vec![] },
                        left_brace: Span::default(),
                        statements: statements.to_vec(),
                        right_brace: Span::default(),
                    },
                };
                *body = MatchArmBody::Expression(Expression::FunctionCall {
                    target: Box::new(Expression::Parenthesized {
                        start: Span::default(),
                        expr: Box::new(Expression::Closure(closure)),
                        end: Span::default(),
                    }),
                    arguments: ArgumentList {
                        comments: CommentGroup { comments: vec![] },
                        left_parenthesis: Span::default(),
                        arguments: vec![],
                        right_parenthesis: Span::default(),
                    },
                })
            }
            _ => {}
        }
    }
}

impl Transpiler for MultiLineMatchTranspiler {
    fn transpile_expression(&mut self, expression: &mut Expression) {
        match expression {
            Expression::Match { default, arms, .. }
            | Expression::ShortMatch { default, arms, .. } => {
                if let Some(default) = default {
                    self.maybe_transpile_match_arm_body(&mut default.body);
                }

                for arm in arms {
                    self.maybe_transpile_match_arm_body(&mut arm.body);
                }
            }
            _ => return,
        }
    }
}
