use crate::Parser;
use crate::ParserDiagnostic;
use pxp_ast::ArrayItemKeyValue;
use pxp_ast::ArrayItemReferencedKeyValue;
use pxp_ast::ArrayItemReferencedValue;
use pxp_ast::ArrayItemSpreadValue;
use pxp_ast::ArrayItemValue;
use pxp_ast::CommentGroup;
use pxp_ast::Expression;
use pxp_ast::ExpressionKind;
use pxp_ast::ListEntry;
use pxp_ast::ListEntryKeyValue;
use pxp_ast::ListEntryValue;
use pxp_ast::{ArrayExpression, ArrayItem, ListExpression, ShortArrayExpression};

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_list_expression(&mut self) -> Expression {
        let list = self.skip(TokenKind::List);
        let start = self.skip_left_parenthesis();
        let items = {
            let mut items = Vec::new();
            let mut has_at_least_one_key = false;

            let mut current = self.current();
            while current.kind != TokenKind::RightParen {
                if current.kind == TokenKind::Comma {
                    self.next();

                    items.push(ListEntry::Skipped(current.span));

                    current = self.current();

                    continue;
                }

                if current.kind == TokenKind::Ellipsis {
                    self.next();

                    self.diagnostic(
                        ParserDiagnostic::InvalidSpreadOperator,
                        Severity::Error,
                        current.span,
                    );
                }

                let mut value = self.parse_expression();
                current = self.current();
                if current.kind == TokenKind::DoubleArrow {
                    if !has_at_least_one_key && !items.is_empty() {
                        self.diagnostic(
                            ParserDiagnostic::CannotMixKeyedAndUnkeyedListEntries,
                            Severity::Error,
                            current.span,
                        );
                    }

                    let double_arrow = current.span;

                    self.next();

                    current = self.current();
                    if current.kind == TokenKind::Ellipsis {
                        self.next();

                        self.diagnostic(
                            ParserDiagnostic::InvalidSpreadOperator,
                            Severity::Error,
                            current.span,
                        );
                    }

                    let mut key = self.parse_expression();
                    current = self.current();

                    std::mem::swap(&mut key, &mut value);

                    items.push(ListEntry::KeyValue(ListEntryKeyValue {
                        id: self.state.id(),
                        span: Span::combine(key.span, value.span),
                        key,
                        double_arrow,
                        value,
                    }));

                    has_at_least_one_key = true;
                } else {
                    if has_at_least_one_key {
                        self.diagnostic(
                            ParserDiagnostic::CannotMixKeyedAndUnkeyedListEntries,
                            Severity::Error,
                            current.span,
                        );
                    }

                    items.push(ListEntry::Value(ListEntryValue {
                        id: self.state.id(),
                        span: value.span,
                        value,
                    }));
                }

                if current.kind == TokenKind::Comma {
                    self.next();
                    current = self.current();
                } else {
                    break;
                }
            }

            if current.kind == TokenKind::Comma {
                self.next();
            }

            items
        };

        let end = self.skip_right_parenthesis();
        let span = Span::combine(list, end);

        let kind = ExpressionKind::List(ListExpression {
            id: self.state.id(),
            span,
            list,
            start,
            items,
            end,
        });

        Expression::new(self.state.id(), kind, span, CommentGroup::default())
    }

    pub fn parse_short_array_expression(&mut self) -> Expression {
        let start = self.skip(TokenKind::LeftBracket);
        let items = self.comma_separated(
            |parser| {
                let current = parser.current();
                if current.kind == TokenKind::Comma {
                    ArrayItem::Skipped(current.span)
                } else {
                    parser.parse_array_pair()
                }
            },
            TokenKind::RightBracket,
        );
        let end = self.skip(TokenKind::RightBracket);
        let span = Span::combine(start, end);

        let kind = ExpressionKind::ShortArray(ShortArrayExpression {
            id: self.state.id(),
            span,
            start,
            items,
            end,
        });

        Expression::new(self.state.id(), kind, span, CommentGroup::default())
    }

    pub fn parse_array_expression(&mut self) -> Expression {
        let array = self.skip(TokenKind::Array);
        let start = self.skip_left_parenthesis();
        let items = self.comma_separated(|parser| parser.parse_array_pair(), TokenKind::RightParen);
        let end = self.skip_right_parenthesis();
        let span = Span::combine(array, end);

        let kind = ExpressionKind::Array(ArrayExpression {
            id: self.state.id(),
            span,
            array,
            start,
            items,
            end,
        });

        Expression::new(self.state.id(), kind, span, CommentGroup::default())
    }

    fn parse_array_pair(&mut self) -> ArrayItem {
        let mut current = self.current();
        let ellipsis = if current.kind == TokenKind::Ellipsis {
            self.next();
            let span = current.span;
            current = self.current();

            Some(span)
        } else {
            None
        };

        let mut ampersand = if current.kind == TokenKind::Ampersand {
            self.next();

            Some(current)
        } else {
            None
        };

        let mut value = self.parse_expression();

        if let Some(ellipsis) = ellipsis {
            if let Some(ampersand) = ampersand {
                self.diagnostic(
                    ParserDiagnostic::UnexpectedToken {
                        token: ampersand.to_owned(),
                    },
                    Severity::Error,
                    ampersand.span,
                );
            }

            return ArrayItem::SpreadValue(ArrayItemSpreadValue {
                id: self.state.id(),
                span: Span::combine(ellipsis, value.span),
                ellipsis,
                value,
            });
        }

        if let Some(ampersand) = ampersand {
            return ArrayItem::ReferencedValue(ArrayItemReferencedValue {
                id: self.state.id(),
                span: Span::combine(ampersand.span, value.span),
                ampersand: ampersand.span,
                value,
            });
        }

        let mut current = self.current();
        if current.kind == TokenKind::DoubleArrow {
            let double_arrow = current.span;

            self.next();

            current = self.current();
            if current.kind == TokenKind::Ellipsis {
                self.next();

                self.diagnostic(
                    ParserDiagnostic::InvalidSpreadOperator,
                    Severity::Error,
                    current.span,
                );
            }

            ampersand = if current.kind == TokenKind::Ampersand {
                self.next();

                Some(current)
            } else {
                None
            };

            let mut key = self.parse_expression();

            std::mem::swap(&mut key, &mut value);

            return match ampersand {
                Some(ampersand) => ArrayItem::ReferencedKeyValue(ArrayItemReferencedKeyValue {
                    id: self.state.id(),
                    span: Span::combine(key.span, value.span),
                    key,
                    double_arrow,
                    value,
                    ampersand: ampersand.span,
                }),
                None => ArrayItem::KeyValue(ArrayItemKeyValue {
                    id: self.state.id(),
                    span: Span::combine(key.span, value.span),
                    key,
                    double_arrow,
                    value,
                }),
            };
        }

        ArrayItem::Value(ArrayItemValue {
            id: self.state.id(),
            span: value.span,
            value,
        })
    }
}
