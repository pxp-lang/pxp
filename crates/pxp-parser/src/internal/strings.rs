use crate::expressions::create;
use crate::internal::identifiers;
use crate::internal::utils;
use crate::internal::variables;
use crate::state::State;
use crate::ParserDiagnostic;
use pxp_ast::*;
use pxp_ast::Expression;
use pxp_ast::ExpressionStringPart;
use pxp_ast::LiteralStringPart;
use pxp_ast::StringPart;
use pxp_ast::{
    ArrayIndexExpression, ExpressionKind, HeredocExpression, InterpolatedStringExpression,
    NowdocExpression, NullsafePropertyFetchExpression, PropertyFetchExpression,
    ShellExecExpression,
};
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_syntax::comments::CommentGroup;
use pxp_token::DocStringIndentationKind;
use pxp_token::TokenKind;

#[inline(always)]
pub fn interpolated(state: &mut State) -> Expression {
    let start_span = state.stream.current().span;
    let mut parts = Vec::new();

    while state.stream.current().kind != TokenKind::DoubleQuote {
        if let Some(part) = part(state) {
            parts.push(part);
        }
    }

    state.stream.next();

    let end_span = state.stream.current().span;

    Expression::new(state.id(),
        ExpressionKind::InterpolatedString(InterpolatedStringExpression { id: state.id(),  span: Span::combine(start_span, end_span), parts }),
        Span::combine(start_span, end_span),
        CommentGroup::default(),
    )
}

#[inline(always)]
pub fn shell_exec(state: &mut State) -> Expression {
    let start_span = state.stream.current().span;
    state.stream.next();

    let mut parts = Vec::new();

    while state.stream.current().kind != TokenKind::Backtick {
        if let Some(part) = part(state) {
            parts.push(part);
        }
    }

    state.stream.next();

    let end_span = state.stream.current().span;

    Expression::new(state.id(),
        ExpressionKind::ShellExec(ShellExecExpression {  id: state.id(), span: Span::combine(start_span, end_span), parts }),
        Span::combine(start_span, end_span),
        CommentGroup::default(),
    )
}

#[inline(always)]
pub fn heredoc(state: &mut State) -> Expression {
    let span = state.stream.current().span;
    let label = state.stream.current().symbol.unwrap();
    state.stream.next();

    let mut parts = Vec::new();

    while !matches!(state.stream.current().kind, TokenKind::EndDocString(_, _)) {
        if let Some(part) = part(state) {
            parts.push(part);
        }
    }

    let (indentation_type, indentation_amount) = match &state.stream.current().kind {
        TokenKind::EndDocString(indentation_type, indentation_amount) => {
            (*indentation_type, *indentation_amount)
        }
        _ => unreachable!(),
    };

    state.stream.next();

    let mut new_line = true;
    if indentation_type != DocStringIndentationKind::None {
        let indentation_char: u8 = indentation_type.into();

        for part in parts.iter_mut() {
            // We only need to strip and validate indentation
            // for individual lines, so we can skip checks if
            // we know we're not on a new line.
            if !new_line {
                continue;
            }

            match part {
                StringPart::Literal(LiteralStringPart { value, .. }) => {
                    let bytes = state.symbol_table.resolve(*value).unwrap();

                    // 1. If this line doesn't start with any whitespace,
                    //    we can return an error early because we know
                    //    the label was indented.
                    if !bytes.starts_with(&[b' ']) && !bytes.starts_with(&[b'\t']) {
                        todo!("tolerant mode")
                        // return Err(SyntaxError::InvalidDocBodyIndentationLevel(
                        //     indentation_amount,
                        //     span,
                        // )
                        // .into());
                    }

                    // 2. If this line doesn't start with the correct
                    //    type of whitespace, we can also return an error.
                    if !bytes.starts_with(&[indentation_char]) {
                        todo!("tolerant mode")
                        // return Err(SyntaxError::InvalidDocIndentation(span).into());
                    }

                    // 3. We now know that the whitespace at the start of
                    //    this line is correct, so we need to check that the
                    //    amount of whitespace is correct too. In this case,
                    //    the amount of whitespace just needs to be at least
                    //    the same, so we can create a vector containing the
                    //    minimum and check using `starts_with()`.
                    let expected_whitespace_buffer = vec![indentation_char; indentation_amount];
                    if !bytes.starts_with(&expected_whitespace_buffer) {
                        todo!("tolerant mode")
                        // return Err(SyntaxError::InvalidDocBodyIndentationLevel(
                        //     indentation_amount,
                        //     span,
                        // )
                        // .into());
                    }

                    // 4. All of the above checks have passed, so we know
                    //    there are no more possible errors. Let's now
                    //    strip the leading whitespace accordingly.

                    // FIXME: Figure out if this is something we can do inside of the lexer instead.
                    // *bytes = ByteStr::new(bytes
                    //     .strip_prefix(&expected_whitespace_buffer[..])
                    //     .unwrap())
                    //     .into();

                    new_line = bytes.ends_with(&[b'\n']);
                }
                _ => continue,
            }
        }
    }

    let end_span = state.stream.previous().span;

    Expression::new(state.id(),
        ExpressionKind::Heredoc(HeredocExpression {  id: state.id(), span: Span::combine(span, end_span), label, parts }),
        Span::combine(span, end_span),
        CommentGroup::default(),
    )
}

#[inline(always)]
pub fn nowdoc(state: &mut State) -> Expression {
    let span = state.stream.current().span;
    let label = *state.stream.current();

    state.stream.next();

    let string_part = *state.stream.current();

    // FIXME: Do we still need this, or can we do it inside of the lexer?
    // let (indentation_type, indentation_amount) = match &state.stream.current().kind {
    //     TokenKind::EndDocString(indentation_type, indentation_amount) => {
    //         (indentation_type.clone(), *indentation_amount)
    //     }
    //     _ => unreachable!(),
    // };

    state.stream.next();

    // FIXME: Figure out if this is something we can do inside of the lexer instead.
    //        If not, then we need to emit diagnostics for invalid indentation etc.
    // if indentation_type != DocStringIndentationKind::None {
    //     let indentation_char: u8 = indentation_type.into();

    //     let mut lines = string_part
    //         .split(|b| *b == b'\n')
    //         .map(|s| s.to_vec())
    //         .collect::<Vec<Vec<u8>>>();

    //     for line in lines.iter_mut() {
    //         if line.is_empty() {
    //             continue;
    //         }

    //         // 1. If this line doesn't start with any whitespace,
    //         //    we can return an error early because we know
    //         //    the label was indented.
    //         if !line.starts_with(&[b' ']) && !line.starts_with(&[b'\t']) {
    //             return Err(
    //                 SyntaxError::InvalidDocBodyIndentationLevel(indentation_amount, span).into(),
    //             );
    //         }

    //         // 2. If this line doesn't start with the correct
    //         //    type of whitespace, we can also return an error.
    //         if !line.starts_with(&[indentation_char]) {
    //             return Err(SyntaxError::InvalidDocIndentation(span).into());
    //         }

    //         // 3. We now know that the whitespace at the start of
    //         //    this line is correct, so we need to check that the
    //         //    amount of whitespace is correct too. In this case,
    //         //    the amount of whitespace just needs to be at least
    //         //    the same, so we can create a vector containing the
    //         //    minimum and check using `starts_with()`.
    //         let expected_whitespace_buffer = vec![indentation_char; indentation_amount];
    //         if !line.starts_with(&expected_whitespace_buffer) {
    //             return Err(
    //                 SyntaxError::InvalidDocBodyIndentationLevel(indentation_amount, span).into(),
    //             );
    //         }

    //         // 4. All of the above checks have passed, so we know
    //         //    there are no more possible errors. Let's now
    //         //    strip the leading whitespace accordingly.
    //         *line = line
    //             .strip_prefix(&expected_whitespace_buffer[..])
    //             .unwrap()
    //             .into();
    //     }

    //     let mut bytes = Vec::new();
    //     for (i, line) in lines.iter().enumerate() {
    //         bytes.extend(line);
    //         if i < lines.len() - 1 {
    //             bytes.push(b'\n');
    //         }
    //     }
    //     string_part = bytes.into();
    // }

    state.stream.next();
    let end_span = state.stream.previous().span;

    Expression::new(state.id(),
        ExpressionKind::Nowdoc(NowdocExpression {
             id: state.id(), 
            span: Span::combine(span, end_span),
            label,
            value: string_part,
        }),
        Span::new(span.start, end_span.end),
        CommentGroup::default(),
    )
}

fn part(state: &mut State) -> Option<StringPart> {
    match &state.stream.current().kind {
        TokenKind::StringPart => {
            let s = *state.stream.current();
            let part = if !s.span.is_empty() {
                Some(StringPart::Literal(LiteralStringPart {
                     id: state.id(), 
                    span: s.span,
                    value: s.symbol.unwrap(),
                }))
            } else {
                None
            };

            state.stream.next();
            part
        }
        TokenKind::DollarLeftBrace => {
            let start_span = state.stream.current().span;
            let variable = variables::dynamic_variable(state);
            let expression = Expression::new(state.id(),
                ExpressionKind::Variable(variable),
                Span::new(start_span.start, state.stream.previous().span.end),
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
            state.stream.next();
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
            let variable_span = state.stream.current().span;
            let variable = ExpressionKind::Variable(variables::dynamic_variable(state));
            let variable =
                Expression::new(state.id(),variable, variable_span, CommentGroup::default());

            let current = state.stream.current();
            let e = match &current.kind {
                TokenKind::LeftBracket => {
                    let left_bracket = utils::skip_left_bracket(state);

                    let current = state.stream.current();
                    let index_start_span = current.span;
                    // Full expression syntax is not allowed here,
                    // so we can't call expression.
                    let index = match &current.kind {
                        TokenKind::LiteralInteger => {
                            state.stream.next();

                            ExpressionKind::Literal(Literal::new(state.id(), LiteralKind::Integer, *current, current.span))
                        }
                        TokenKind::Minus => {
                            state.stream.next();
                            let literal = state.stream.current();
                            if let TokenKind::LiteralInteger = &literal.kind {
                                let span = state.stream.current().span;
                                state.stream.next();
                                let kind = ExpressionKind::Literal(Literal::new(
                                    state.id(),
                                    LiteralKind::Integer,
                                    *literal,
                                    literal.span,
                                ));
                                let expression = Expression::new(state.id(),
                                    kind,
                                    span,
                                    CommentGroup::default(),
                                );

                                ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression {
                                     id: state.id(), 
                                    span: Span::combine(span, expression.span),
                                    kind: ArithmeticOperationKind::Negative {
                                        minus: span,
                                        right: Box::new(expression),
                                    }
                                })
                            } else {
                                state.diagnostic(
                                    ParserDiagnostic::ExpectedToken {
                                        expected: vec![TokenKind::LiteralInteger],
                                        found: *literal,
                                    },
                                    Severity::Error,
                                    literal.span,
                                );

                                state.stream.next();

                                ExpressionKind::Missing(literal.span)
                            }
                        }
                        TokenKind::Identifier => {
                            state.stream.next();

                            ExpressionKind::Literal(Literal::new(state.id(), LiteralKind::String, *current, current.span))
                        }
                        TokenKind::Variable => ExpressionKind::Variable(Variable::SimpleVariable(
                            variables::simple_variable(state),
                        )),
                        _ => {
                            state.diagnostic(
                                ParserDiagnostic::ExpectedToken {
                                    expected: vec![
                                        TokenKind::LiteralInteger,
                                        TokenKind::Identifier,
                                        TokenKind::Variable,
                                    ],
                                    found: *current,
                                },
                                Severity::Error,
                                current.span,
                            );

                            state.stream.next();

                            ExpressionKind::Missing(current.span)
                        }
                    };
                    let index_end_span = state.stream.previous().span;
                    let index = Expression::new(state.id(),
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

                    state.stream.next();

                    let identifier = identifiers::identifier_maybe_reserved(state);
                    let id_span = identifier.span;
                    let kind = ExpressionKind::Identifier(Identifier::SimpleIdentifier(identifier));
                    let identifier_expression =
                        Expression::new(state.id(),kind, id_span, CommentGroup::default());

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
                    state.stream.next();

                    let ident = identifiers::identifier_maybe_reserved(state);
                    let ident_span = ident.span;
                    let kind = ExpressionKind::Identifier(Identifier::SimpleIdentifier(ident));

                    ExpressionKind::NullsafePropertyFetch(NullsafePropertyFetchExpression {
                         id: state.id(), 
                        span: Span::combine(variable.span, ident_span),
                        target: Box::new(variable),
                        question_arrow: span,
                        property: Box::new(Expression::new(state.id(),
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
                span: Span::combine(variable_span, state.stream.previous().span),
                expression: Box::new(Expression::new(state.id(),
                    e,
                    Span::new(variable_span.start, state.stream.previous().span.end),
                    CommentGroup::default(),
                )),
            }))
        }
        _ => {
            let span = state.stream.current().span;

            state.diagnostic(
                ParserDiagnostic::ExpectedToken {
                    expected: vec![TokenKind::LeftBrace, TokenKind::DollarLeftBrace, TokenKind::DoubleQuote, TokenKind::Variable],
                    found: *state.stream.current(),
                },
                Severity::Error,
                span
            );

            state.stream.next();

            None
        }
    }
}
