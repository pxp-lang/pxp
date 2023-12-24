use crate::expressions;
use crate::internal::utils;
use crate::state::State;
use pxp_ast::Expression;
use pxp_ast::ExpressionKind;
use pxp_ast::ListEntry;
use pxp_ast::{ArrayExpression, ArrayItem, ListExpression, ShortArrayExpression};
use pxp_diagnostics::DiagnosticKind;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_syntax::comments::CommentGroup;
use pxp_token::TokenKind;

pub fn list_expression(state: &mut State) -> Expression {
    let start_span = state.stream.current().span;
    let kind = ExpressionKind::List(ListExpression {
        list: utils::skip(state, TokenKind::List),
        start: utils::skip_left_parenthesis(state),
        items: {
            let mut items = Vec::new();
            let mut has_at_least_one_key = false;

            let mut current = state.stream.current();
            while current.kind != TokenKind::RightParen {
                if current.kind == TokenKind::Comma {
                    state.stream.next();

                    items.push(ListEntry::Skipped);

                    current = state.stream.current();

                    continue;
                }

                if current.kind == TokenKind::Ellipsis {
                    state.stream.next();

                    state.diagnostic(
                        DiagnosticKind::InvalidSpreadOperator,
                        Severity::Error,
                        current.span,
                    );
                }

                let mut value = expressions::create(state);
                current = state.stream.current();
                if current.kind == TokenKind::DoubleArrow {
                    if !has_at_least_one_key && !items.is_empty() {
                        state.diagnostic(
                            DiagnosticKind::CannotMixKeyedAndUnkeyedListEntries,
                            Severity::Error,
                            current.span,
                        );
                    }

                    let double_arrow = current.span;

                    state.stream.next();

                    current = state.stream.current();
                    if current.kind == TokenKind::Ellipsis {
                        state.stream.next();

                        state.diagnostic(
                            DiagnosticKind::InvalidSpreadOperator,
                            Severity::Error,
                            current.span,
                        );
                    }

                    let mut key = expressions::create(state);
                    current = state.stream.current();

                    std::mem::swap(&mut key, &mut value);

                    items.push(ListEntry::KeyValue {
                        key,
                        double_arrow,
                        value,
                    });

                    has_at_least_one_key = true;
                } else {
                    if has_at_least_one_key {
                        state.diagnostic(
                            DiagnosticKind::CannotMixKeyedAndUnkeyedListEntries,
                            Severity::Error,
                            current.span,
                        );
                    }

                    items.push(ListEntry::Value { value });
                }

                if current.kind == TokenKind::Comma {
                    state.stream.next();
                    current = state.stream.current();
                } else {
                    break;
                }
            }

            if current.kind == TokenKind::Comma {
                state.stream.next();
            }

            items
        },
        end: utils::skip_right_parenthesis(state),
    });
    let end_span = state.stream.current().span;

    Expression::new(
        state.id(),
        kind,
        Span::new(start_span.start, end_span.end),
        CommentGroup::default(),
    )
}

pub fn short_array_expression(state: &mut State) -> Expression {
    let start_span = state.stream.current().span;
    let kind = ExpressionKind::ShortArray(ShortArrayExpression {
        start: utils::skip(state, TokenKind::LeftBracket),
        items: utils::comma_separated(
            state,
            &|state| {
                let current = state.stream.current();
                if current.kind == TokenKind::Comma {
                    ArrayItem::Skipped
                } else {
                    array_pair(state)
                }
            },
            TokenKind::RightBracket,
        ),
        end: utils::skip(state, TokenKind::RightBracket),
    });
    let end_span = state.stream.current().span;

    Expression::new(
        state.id(),
        kind,
        Span::new(start_span.start, end_span.end),
        CommentGroup::default(),
    )
}

pub fn array_expression(state: &mut State) -> Expression {
    let start_span = state.stream.current().span;
    let kind = ExpressionKind::Array(ArrayExpression {
        array: utils::skip(state, TokenKind::Array),
        start: utils::skip_left_parenthesis(state),
        items: utils::comma_separated(state, &array_pair, TokenKind::RightParen),
        end: utils::skip_right_parenthesis(state),
    });
    let end_span = state.stream.current().span;

    Expression::new(
        state.id(),
        kind,
        Span::new(start_span.start, end_span.end),
        CommentGroup::default(),
    )
}

fn array_pair(state: &mut State) -> ArrayItem {
    let mut current = state.stream.current();
    let ellipsis = if current.kind == TokenKind::Ellipsis {
        state.stream.next();
        let span = current.span;
        current = state.stream.current();

        Some(span)
    } else {
        None
    };

    let mut ampersand = if current.kind == TokenKind::Ampersand {
        state.stream.next();

        Some(current)
    } else {
        None
    };

    let mut value = expressions::create(state);

    if let Some(ellipsis) = ellipsis {
        if let Some(ampersand) = ampersand {
            state.diagnostic(
                DiagnosticKind::UnexpectedToken { token: *ampersand },
                Severity::Error,
                ampersand.span,
            );
        }

        return ArrayItem::SpreadValue { ellipsis, value };
    }

    if let Some(ampersand) = ampersand {
        return ArrayItem::ReferencedValue {
            ampersand: ampersand.span,
            value,
        };
    }

    let mut current = state.stream.current();
    if current.kind == TokenKind::DoubleArrow {
        let double_arrow = current.span;

        state.stream.next();

        current = state.stream.current();
        if current.kind == TokenKind::Ellipsis {
            state.stream.next();

            state.diagnostic(
                DiagnosticKind::InvalidSpreadOperator,
                Severity::Error,
                current.span,
            );
        }

        ampersand = if current.kind == TokenKind::Ampersand {
            state.stream.next();

            Some(current)
        } else {
            None
        };

        let mut key = expressions::create(state);

        std::mem::swap(&mut key, &mut value);

        return match ampersand {
            Some(ampersand) => ArrayItem::ReferencedKeyValue {
                key,
                double_arrow,
                value,
                ampersand: ampersand.span,
            },
            None => ArrayItem::KeyValue {
                key,
                double_arrow,
                value,
            },
        };
    }

    ArrayItem::Value { value }
}
