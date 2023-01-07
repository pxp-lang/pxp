use pxp_parser::{
    lexer::token::Span,
    parser::ast::{
        arguments::{Argument, ArgumentList},
        comments::CommentGroup,
        identifiers::{Identifier, SimpleIdentifier},
        operators::RangeOperation,
        Expression,
    },
};

use super::Transpiler;

pub struct RangeTranspiler;

impl Transpiler for RangeTranspiler {
    fn transpile_expression(&mut self, expression: &mut Expression) {
        if let Expression::RangeOperation(operation) = expression {
            *expression = match operation {
                RangeOperation::Exclusive {
                    lower_bound,
                    upper_bound,
                    ..
                } => Expression::New {
                    new: Span::default(),
                    target: Box::new(Expression::Identifier(Identifier::SimpleIdentifier(
                        SimpleIdentifier {
                            span: Span::default(),
                            value: "\\Pxp\\Runtime\\Range\\ExclusiveRange".into(),
                        },
                    ))),
                    arguments: Some(ArgumentList {
                        comments: CommentGroup { comments: vec![] },
                        left_parenthesis: Span::default(),
                        arguments: vec![
                            Argument::Positional {
                                comments: CommentGroup { comments: vec![] },
                                ellipsis: None,
                                value: *lower_bound.clone(),
                            },
                            Argument::Positional {
                                comments: CommentGroup { comments: vec![] },
                                ellipsis: None,
                                value: *upper_bound.clone(),
                            },
                        ],
                        right_parenthesis: Span::default(),
                    }),
                },
                RangeOperation::Inclusive {
                    lower_bound,
                    upper_bound,
                    ..
                } => Expression::New {
                    new: Span::default(),
                    target: Box::new(Expression::Identifier(Identifier::SimpleIdentifier(
                        SimpleIdentifier {
                            span: Span::default(),
                            value: "\\Pxp\\Runtime\\Range\\InclusiveRange".into(),
                        },
                    ))),
                    arguments: Some(ArgumentList {
                        comments: CommentGroup { comments: vec![] },
                        left_parenthesis: Span::default(),
                        arguments: vec![
                            Argument::Positional {
                                comments: CommentGroup { comments: vec![] },
                                ellipsis: None,
                                value: *lower_bound.clone(),
                            },
                            Argument::Positional {
                                comments: CommentGroup { comments: vec![] },
                                ellipsis: None,
                                value: *upper_bound.clone(),
                            },
                        ],
                        right_parenthesis: Span::default(),
                    }),
                },
                RangeOperation::Endless { lower_bound, .. } => Expression::New {
                    new: Span::default(),
                    target: Box::new(Expression::Identifier(Identifier::SimpleIdentifier(
                        SimpleIdentifier {
                            span: Span::default(),
                            value: "\\Pxp\\Runtime\\Range\\EndlessRange".into(),
                        },
                    ))),
                    arguments: Some(ArgumentList {
                        comments: CommentGroup { comments: vec![] },
                        left_parenthesis: Span::default(),
                        arguments: vec![Argument::Positional {
                            comments: CommentGroup { comments: vec![] },
                            ellipsis: None,
                            value: *lower_bound.clone(),
                        }],
                        right_parenthesis: Span::default(),
                    }),
                },
            };
        }
    }
}
