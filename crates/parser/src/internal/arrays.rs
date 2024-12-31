use crate::internal::diagnostics::ParserDiagnostic;
use crate::Parser;
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

            while !self.is_eof() && self.current_kind() != TokenKind::RightParen {
                if self.current_kind() == TokenKind::Comma {
                    let span = self.next();

                    items.push(ListEntry::Skipped(span));

                    continue;
                }

                if self.current_kind() == TokenKind::Ellipsis {
                    let span = self.next();

                    self.diagnostic(
                        ParserDiagnostic::InvalidSpreadOperator,
                        Severity::Error,
                        span,
                    );
                }

                let mut value = self.parse_expression();

                if self.current_kind() == TokenKind::DoubleArrow {
                    if !has_at_least_one_key && !items.is_empty() {
                        self.diagnostic(
                            ParserDiagnostic::CannotMixKeyedAndUnkeyedListEntries,
                            Severity::Error,
                            self.current_span(),
                        );
                    }

                    let double_arrow = self.next();

                    if self.current_kind() == TokenKind::Ellipsis {
                        let span = self.next();

                        self.diagnostic(
                            ParserDiagnostic::InvalidSpreadOperator,
                            Severity::Error,
                            span,
                        );
                    }

                    let mut key = self.parse_expression();

                    std::mem::swap(&mut key, &mut value);

                    items.push(ListEntry::KeyValue(ListEntryKeyValue {
                        id: self.id(),
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
                            self.current_span(),
                        );
                    }

                    items.push(ListEntry::Value(ListEntryValue {
                        id: self.id(),
                        span: value.span,
                        value,
                    }));
                }

                if self.current_kind() == TokenKind::Comma {
                    self.next();
                } else {
                    break;
                }
            }

            if self.current_kind() == TokenKind::Comma {
                self.next();
            }

            items
        };

        let end = self.skip_right_parenthesis();
        let span = Span::combine(list, end);

        let kind = ExpressionKind::List(Box::new(ListExpression {
            id: self.id(),
            span,
            list,
            start,
            items,
            end,
        }));

        Expression::new(self.id(), kind, span, CommentGroup::default())
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

        let kind = ExpressionKind::ShortArray(Box::new(ShortArrayExpression {
            id: self.id(),
            span,
            start,
            items,
            end,
        }));

        Expression::new(self.id(), kind, span, CommentGroup::default())
    }

    pub fn parse_array_expression(&mut self) -> Expression {
        let array = self.skip(TokenKind::Array);
        let start = self.skip_left_parenthesis();
        let items = self.comma_separated(|parser| parser.parse_array_pair(), TokenKind::RightParen);
        let end = self.skip_right_parenthesis();
        let span = Span::combine(array, end);

        let kind = ExpressionKind::Array(Box::new(ArrayExpression {
            id: self.id(),
            span,
            array,
            start,
            items,
            end,
        }));

        Expression::new(self.id(), kind, span, CommentGroup::default())
    }

    fn parse_array_pair(&mut self) -> ArrayItem {
        let ellipsis = self.optional(TokenKind::Ellipsis);

        let mut ampersand = if self.current_kind() == TokenKind::Ampersand {
            if ellipsis.is_some() {
                self.diagnostic(
                    ParserDiagnostic::UnexpectedToken {
                        token: self.current().to_owned(),
                    },
                    Severity::Error,
                    self.current_span(),
                );
            }

            Some(self.next())
        } else {
            None
        };

        let mut value = self.parse_expression();

        if let Some(ellipsis) = ellipsis {
            return ArrayItem::SpreadValue(ArrayItemSpreadValue {
                id: self.id(),
                span: Span::combine(ellipsis, value.span),
                ellipsis,
                value,
            });
        }

        if let Some(ampersand) = ampersand {
            return ArrayItem::ReferencedValue(ArrayItemReferencedValue {
                id: self.id(),
                span: Span::combine(ampersand, value.span),
                ampersand,
                value,
            });
        }

        if self.current_kind() == TokenKind::DoubleArrow {
            let double_arrow = self.next();

            if self.current_kind() == TokenKind::Ellipsis {
                let span = self.next();

                self.diagnostic(
                    ParserDiagnostic::InvalidSpreadOperator,
                    Severity::Error,
                    span,
                );
            }

            ampersand = self.optional(TokenKind::Ampersand);

            let mut key = self.parse_expression();

            std::mem::swap(&mut key, &mut value);

            return match ampersand {
                Some(ampersand) => ArrayItem::ReferencedKeyValue(ArrayItemReferencedKeyValue {
                    id: self.id(),
                    span: Span::combine(key.span, value.span),
                    key,
                    double_arrow,
                    value,
                    ampersand,
                }),
                None => ArrayItem::KeyValue(ArrayItemKeyValue {
                    id: self.id(),
                    span: Span::combine(key.span, value.span),
                    key,
                    double_arrow,
                    value,
                }),
            };
        }

        ArrayItem::Value(ArrayItemValue {
            id: self.id(),
            span: value.span,
            value,
        })
    }
}
