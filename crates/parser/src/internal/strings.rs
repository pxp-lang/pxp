use crate::expressions::create;
use crate::internal::identifiers;
use crate::internal::utils;
use crate::internal::variables;
use crate::state::State;
use crate::Parser;
use crate::ParserDiagnostic;
use pxp_ast::Expression;
use pxp_ast::ExpressionStringPart;
use pxp_ast::LiteralStringPart;
use pxp_ast::StringPart;
use pxp_ast::*;
use pxp_ast::{
    ArrayIndexExpression, ExpressionKind, HeredocExpression, InterpolatedStringExpression,
    NowdocExpression, NullsafePropertyFetchExpression, PropertyFetchExpression,
    ShellExecExpression,
};
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    #[inline(always)]
    pub fn parse_interpolated_string(&mut self) -> Expression {
        let start_span = state.current().span;
        let mut parts = Vec::new();

        while state.current().kind != TokenKind::DoubleQuote {
            if let Some(part) = maybe_parse_string_part(state) {
                parts.push(part);
            }
        }

        state.next();

        let end_span = state.current().span;

        Expression::new(
            state.id(),
            ExpressionKind::InterpolatedString(InterpolatedStringExpression {
                id: state.id(),
                span: Span::combine(start_span, end_span),
                parts,
            }),
            Span::combine(start_span, end_span),
            CommentGroup::default(),
        )
    }

    #[inline(always)]
    pub fn parse_shell_exec_string(&mut self) -> Expression {
        let start_span = state.current().span;
        state.next();

        let mut parts = Vec::new();

        while state.current().kind != TokenKind::Backtick {
            if let Some(part) = maybe_parse_string_part(state) {
                parts.push(part);
            }
        }

        state.next();

        let end_span = state.current().span;

        Expression::new(
            state.id(),
            ExpressionKind::ShellExec(ShellExecExpression {
                id: state.id(),
                span: Span::combine(start_span, end_span),
                parts,
            }),
            Span::combine(start_span, end_span),
            CommentGroup::default(),
        )
    }

    #[inline(always)]
    pub fn parse_heredoc(&mut self) -> Expression {
        let span = state.current().span;
        let label = state.current().symbol.as_ref().unwrap();
        state.next();

        let mut parts = Vec::new();

        while !matches!(state.current().kind, TokenKind::EndHeredoc) {
            if let Some(part) = maybe_parse_string_part(state) {
                parts.push(part);
            }
        }

        let end = state.current();

        state.next();

        Expression::new(
            state.id(),
            ExpressionKind::Heredoc(HeredocExpression {
                id: state.id(),
                span: Span::combine(span, end.span),
                label: label.clone(),
                parts,
            }),
            Span::combine(span, end.span),
            CommentGroup::default(),
        )
    }

    #[inline(always)]
    pub fn parse_nowdoc(&mut self) -> Expression {
        let span = state.current().span;
        let label = state.current().clone();

        state.next();

        let string_part = state.current().clone();

        state.next();

        let end = state.current();

        let span = if !state.is_eof() && end.kind != TokenKind::EndNowdoc {
            state.diagnostic(
                ParserDiagnostic::ExpectedToken {
                    expected: vec![TokenKind::EndNowdoc],
                    found: end.clone(),
                },
                Severity::Error,
                end.span,
            );

            span
        } else if end.kind == TokenKind::EndNowdoc {
            state.next();

            Span::combine(span, end.span)
        } else {
            span
        };

        Expression::new(
            state.id(),
            ExpressionKind::Nowdoc(NowdocExpression {
                id: state.id(),
                span,
                label,
                value: string_part,
            }),
            span,
            CommentGroup::default(),
        )
    }

    fn maybe_parse_string_part(&mut self) -> Option<StringPart> {
        match &state.current().kind {
            TokenKind::StringPart => {
                let s = state.current().clone();
                let part = if !s.span.is_empty() {
                    Some(StringPart::Literal(LiteralStringPart {
                        id: state.id(),
                        span: s.span,
                        value: s.symbol.unwrap(),
                    }))
                } else {
                    None
                };

                state.next();
                part
            }
            TokenKind::DollarLeftBrace => {
                let start_span = state.current().span;
                let variable = variables::parse_dynamic_variable(state);
                let expression = Expression::new(
                    state.id(),
                    ExpressionKind::Variable(variable),
                    Span::new(start_span.start, state.previous().span.end),
                    CommentGroup::default(),
                );

                Some(StringPart::Expression(ExpressionStringPart {
                    id: state.id(),
                    span: expression.span,
                    expression: Box::new(expression),
                }))
            }
            TokenKind::LeftBrace => {
                // "{$expr}"
                state.next();
                let e = create(state);
                utils::skip_right_brace(state);
                Some(StringPart::Expression(ExpressionStringPart {
                    id: state.id(),
                    span: e.span,
                    expression: Box::new(e),
                }))
            }
            TokenKind::Variable => {
                // "$expr", "$expr[0]", "$expr[name]", "$expr->a"
                let variable_span = state.current().span;
                let variable = ExpressionKind::Variable(variables::parse_dynamic_variable(state));
                let variable =
                    Expression::new(state.id(), variable, variable_span, CommentGroup::default());

                let current = state.current();
                let e = match &current.kind {
                    TokenKind::LeftBracket => {
                        let left_bracket = utils::skip_left_bracket(state);

                        let current = state.current();
                        let index_start_span = current.span;
                        // Full expression syntax is not allowed here,
                        // so we can't call expression.
                        let index = match &current.kind {
                            TokenKind::LiteralInteger => {
                                state.next();

                                ExpressionKind::Literal(Literal::new(
                                    state.id(),
                                    LiteralKind::Integer,
                                    current.clone(),
                                    current.span,
                                ))
                            }
                            TokenKind::Minus => {
                                state.next();
                                let literal = state.current();
                                if let TokenKind::LiteralInteger = &literal.kind {
                                    let span = state.current().span;
                                    state.next();
                                    let kind = ExpressionKind::Literal(Literal::new(
                                        state.id(),
                                        LiteralKind::Integer,
                                        literal.clone(),
                                        literal.span,
                                    ));
                                    let expression = Expression::new(
                                        state.id(),
                                        kind,
                                        span,
                                        CommentGroup::default(),
                                    );

                                    ExpressionKind::ArithmeticOperation(
                                        ArithmeticOperationExpression {
                                            id: state.id(),
                                            span: Span::combine(span, expression.span),
                                            kind: ArithmeticOperationKind::Negative {
                                                id: state.id(),
                                                minus: span,
                                                right: Box::new(expression),
                                            },
                                        },
                                    )
                                } else {
                                    state.diagnostic(
                                        ParserDiagnostic::ExpectedToken {
                                            expected: vec![TokenKind::LiteralInteger],
                                            found: literal.clone(),
                                        },
                                        Severity::Error,
                                        literal.span,
                                    );

                                    state.next();

                                    ExpressionKind::Missing(MissingExpression {
                                        id: 0,
                                        span: literal.span,
                                    })
                                }
                            }
                            TokenKind::Identifier => {
                                state.next();

                                ExpressionKind::Literal(Literal::new(
                                    state.id(),
                                    LiteralKind::String,
                                    current.clone(),
                                    current.span,
                                ))
                            }
                            TokenKind::Variable => ExpressionKind::Variable(
                                Variable::SimpleVariable(variables::parse_simple_variable(state)),
                            ),
                            _ => {
                                state.diagnostic(
                                    ParserDiagnostic::ExpectedToken {
                                        expected: vec![
                                            TokenKind::LiteralInteger,
                                            TokenKind::Identifier,
                                            TokenKind::Variable,
                                        ],
                                        found: current.clone(),
                                    },
                                    Severity::Error,
                                    current.span,
                                );

                                state.next();

                                ExpressionKind::Missing(MissingExpression {
                                    id: 0,
                                    span: current.span,
                                })
                            }
                        };
                        let index_end_span = state.previous().span;
                        let index = Expression::new(
                            state.id(),
                            index,
                            Span::new(index_start_span.start, index_end_span.end),
                            CommentGroup::default(),
                        );

                        let right_bracket = utils::skip_right_bracket(state);

                        ExpressionKind::ArrayIndex(ArrayIndexExpression {
                            id: state.id(),
                            span: Span::combine(variable.span, right_bracket),
                            array: Box::new(variable),
                            left_bracket,
                            index: Some(Box::new(index)),
                            right_bracket,
                        })
                    }
                    TokenKind::Arrow => {
                        let span = current.span;

                        state.next();

                        let identifier = identifiers::parse_identifier_maybe_reserved(state);
                        let id_span = identifier.span;
                        let kind =
                            ExpressionKind::Identifier(Identifier::SimpleIdentifier(identifier));
                        let identifier_expression =
                            Expression::new(state.id(), kind, id_span, CommentGroup::default());

                        ExpressionKind::PropertyFetch(PropertyFetchExpression {
                            id: state.id(),
                            span: Span::combine(variable.span, identifier_expression.span),
                            target: Box::new(variable),
                            arrow: span,
                            property: Box::new(identifier_expression),
                        })
                    }
                    TokenKind::QuestionArrow => {
                        let span = current.span;
                        state.next();

                        let ident = identifiers::parse_identifier_maybe_reserved(state);
                        let ident_span = ident.span;
                        let kind = ExpressionKind::Identifier(Identifier::SimpleIdentifier(ident));

                        ExpressionKind::NullsafePropertyFetch(NullsafePropertyFetchExpression {
                            id: state.id(),
                            span: Span::combine(variable.span, ident_span),
                            target: Box::new(variable),
                            question_arrow: span,
                            property: Box::new(Expression::new(
                                state.id(),
                                kind,
                                ident_span,
                                CommentGroup::default(),
                            )),
                        })
                    }
                    // FIXME: This is hacky and bad for performance & memory, but works for now.
                    _ => variable.kind.clone(),
                };

                Some(StringPart::Expression(ExpressionStringPart {
                    id: state.id(),
                    span: Span::combine(variable_span, state.previous().span),
                    expression: Box::new(Expression::new(
                        state.id(),
                        e,
                        Span::new(variable_span.start, state.previous().span.end),
                        CommentGroup::default(),
                    )),
                }))
            }
            _ => {
                let span = state.current().span;

                state.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![
                            TokenKind::LeftBrace,
                            TokenKind::DollarLeftBrace,
                            TokenKind::DoubleQuote,
                            TokenKind::Variable,
                        ],
                        found: state.current().clone(),
                    },
                    Severity::Error,
                    span,
                );

                state.next();

                None
            }
        }
    }
}
