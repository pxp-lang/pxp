use crate::expressions;
use crate::internal::utils;
use crate::state::State;
use crate::ParserDiagnostic;
use pxp_ast::ArrayItemKeyValue;
use pxp_ast::ArrayItemReferencedKeyValue;
use pxp_ast::ArrayItemReferencedValue;
use pxp_ast::ArrayItemSpreadValue;
use pxp_ast::ArrayItemValue;
use pxp_ast::Expression;
use pxp_ast::ExpressionKind;
use pxp_ast::ListEntry;
use pxp_ast::ListEntryKeyValue;
use pxp_ast::ListEntryValue;
use pxp_ast::{ArrayExpression, ArrayItem, ListExpression, ShortArrayExpression};

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_syntax::comments::CommentGroup;
use pxp_token::TokenKind;

pub fn list_expression(state: &mut State) -> Expression {
    let list = utils::skip(state, TokenKind::List);
    let start = utils::skip_left_parenthesis(state);
    let items = {
        let mut items = Vec::new();
        let mut has_at_least_one_key = false;

        let mut current = state.stream.current();
        while current.kind != TokenKind::RightParen {
            if current.kind == TokenKind::Comma {
                state.stream.next();

                items.push(ListEntry::Skipped(current.span));

                current = state.stream.current();

                continue;
            }

            if current.kind == TokenKind::Ellipsis {
                state.stream.next();

                state.diagnostic(
                    ParserDiagnostic::InvalidSpreadOperator,
                    Severity::Error,
                    current.span,
                );
            }

            let mut value = expressions::create(state);
            current = state.stream.current();
            if current.kind == TokenKind::DoubleArrow {
                if !has_at_least_one_key && !items.is_empty() {
                    state.diagnostic(
                        ParserDiagnostic::CannotMixKeyedAndUnkeyedListEntries,
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
                        ParserDiagnostic::InvalidSpreadOperator,
                        Severity::Error,
                        current.span,
                    );
                }

                let mut key = expressions::create(state);
                current = state.stream.current();

                std::mem::swap(&mut key, &mut value);

                items.push(ListEntry::KeyValue(ListEntryKeyValue {
                    span: Span::combine(key.span, value.span),
                    key,
                    double_arrow,
                    value,
                }));

                has_at_least_one_key = true;
            } else {
                if has_at_least_one_key {
                    state.diagnostic(
                        ParserDiagnostic::CannotMixKeyedAndUnkeyedListEntries,
                        Severity::Error,
                        current.span,
                    );
                }

                items.push(ListEntry::Value(ListEntryValue { span: value.span, value }));
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
    };

    let end = utils::skip_right_parenthesis(state);
    let span = Span::combine(list, end);

    let kind = ExpressionKind::List(ListExpression {
        span,
        list,
        start,
        items,
        end,
    });

    Expression::new(
        kind,
        span,
        CommentGroup::default(),
    )
}

pub fn short_array_expression(state: &mut State) -> Expression {
    let start = utils::skip(state, TokenKind::LeftBracket);
    let items = utils::comma_separated(
        state,
        &|state| {
            let current = state.stream.current();
            if current.kind == TokenKind::Comma {
                ArrayItem::Skipped(current.span)
            } else {
                array_pair(state)
            }
        },
        TokenKind::RightBracket,
    );
    let end = utils::skip(state, TokenKind::RightBracket);
    let span = Span::combine(start, end);

    let kind = ExpressionKind::ShortArray(ShortArrayExpression {
        span,
        start,
        items,
        end,
    });

    Expression::new(
        kind,
        span,
        CommentGroup::default(),
    )
}

pub fn array_expression(state: &mut State) -> Expression {
    let array = utils::skip(state, TokenKind::Array);
    let start = utils::skip_left_parenthesis(state);
    let items = utils::comma_separated(state, &array_pair, TokenKind::RightParen);
    let end = utils::skip_right_parenthesis(state);
    let span = Span::combine(array, end);

    let kind = ExpressionKind::Array(ArrayExpression {
        span,
        array,
        start,
        items,
        end,
    });

    Expression::new(
        kind,
        span,
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
                ParserDiagnostic::UnexpectedToken { token: *ampersand },
                Severity::Error,
                ampersand.span,
            );
        }

        return ArrayItem::SpreadValue(ArrayItemSpreadValue { span: Span::combine(ellipsis, value.span), ellipsis, value });
    }

    if let Some(ampersand) = ampersand {
        return ArrayItem::ReferencedValue(ArrayItemReferencedValue {
            span: Span::combine(ampersand.span, value.span),
            ampersand: ampersand.span,
            value,
        });
    }

    let mut current = state.stream.current();
    if current.kind == TokenKind::DoubleArrow {
        let double_arrow = current.span;

        state.stream.next();

        current = state.stream.current();
        if current.kind == TokenKind::Ellipsis {
            state.stream.next();

            state.diagnostic(
                ParserDiagnostic::InvalidSpreadOperator,
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
            Some(ampersand) => ArrayItem::ReferencedKeyValue(ArrayItemReferencedKeyValue {
                span: Span::combine(key.span, value.span),
                key,
                double_arrow,
                value,
                ampersand: ampersand.span,
            }),
            None => ArrayItem::KeyValue(ArrayItemKeyValue {
                span: Span::combine(key.span, value.span),
                key,
                double_arrow,
                value,
            }),
        };
    }

    ArrayItem::Value(ArrayItemValue { span: value.span, value })
}
