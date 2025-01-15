use crate::internal::diagnostics::ParserDiagnostic;
use crate::Parser;
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
use pxp_span::IsSpanned;
use pxp_span::Span;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    #[inline(always)]
    pub fn parse_interpolated_string(&mut self) -> Expression {
        let start_span = self.current_span();
        let mut parts = Vec::new();

        while !self.is_eof() && self.current_kind() != TokenKind::DoubleQuote {
            if let Some(part) = self.maybe_parse_string_part() {
                parts.push(part);
            }
        }

        self.next();

        let end_span = self.current_span();

        Expression::new(
            self.id(),
            ExpressionKind::InterpolatedString(Box::new(InterpolatedStringExpression {
                id: self.id(),
                span: Span::combine(start_span, end_span),
                parts,
            })),
            Span::combine(start_span, end_span),
            CommentGroup::default(),
        )
    }

    #[inline(always)]
    pub fn parse_shell_exec_string(&mut self) -> Expression {
        let start_span = self.current_span();
        self.next();

        let mut parts = Vec::new();

        while !self.is_eof() && self.current_kind() != TokenKind::Backtick {
            if let Some(part) = self.maybe_parse_string_part() {
                parts.push(part);
            }
        }

        self.next();

        let end_span = self.current_span();

        Expression::new(
            self.id(),
            ExpressionKind::ShellExec(Box::new(ShellExecExpression {
                id: self.id(),
                span: Span::combine(start_span, end_span),
                parts,
            })),
            Span::combine(start_span, end_span),
            CommentGroup::default(),
        )
    }

    #[inline(always)]
    pub fn parse_heredoc(&mut self) -> Expression {
        let span = self.current_span();
        let label = self.current_symbol_as_bytestring();

        self.next();

        let mut parts = Vec::new();

        while !matches!(self.current_kind(), TokenKind::EndHeredoc) {
            if let Some(part) = self.maybe_parse_string_part() {
                parts.push(part);
            }
        }

        let end = self.next();

        Expression::new(
            self.id(),
            ExpressionKind::Heredoc(Box::new(HeredocExpression {
                id: self.id(),
                span: Span::combine(span, end),
                label: label.clone(),
                parts,
            })),
            Span::combine(span, end),
            CommentGroup::default(),
        )
    }

    #[inline(always)]
    pub fn parse_nowdoc(&mut self) -> Expression {
        let span = self.current_span();
        let label = self.current().to_owned();

        self.next();

        let string_part = self.current().to_owned();

        self.next();

        let span = if !self.is_eof() && self.current_kind() != TokenKind::EndNowdoc {
            self.diagnostic(
                ParserDiagnostic::ExpectedToken {
                    expected: vec![TokenKind::EndNowdoc],
                    found: self.current().to_owned(),
                },
                Severity::Error,
                self.current_span(),
            );

            span
        } else if self.current_kind() == TokenKind::EndNowdoc {
            Span::combine(span, self.next())
        } else {
            span
        };

        Expression::new(
            self.id(),
            ExpressionKind::Nowdoc(Box::new(NowdocExpression {
                id: self.id(),
                span,
                label,
                value: string_part,
            })),
            span,
            CommentGroup::default(),
        )
    }

    fn maybe_parse_string_part(&mut self) -> Option<StringPart> {
        match &self.current_kind() {
            TokenKind::StringPart => {
                let span = self.current_span();

                let part = if !span.is_empty() {
                    Some(StringPart::Literal(LiteralStringPart {
                        id: self.id(),
                        span,
                        value: self.current_symbol_as_bytestring(),
                    }))
                } else {
                    None
                };

                self.next();
                part
            }
            TokenKind::DollarLeftBrace => {
                let start_span = self.current_span();
                let variable = self.parse_dynamic_variable();
                let span = Span::combine(start_span, variable.span());

                let expression = Expression::new(
                    self.id(),
                    ExpressionKind::Variable(Box::new(variable)),
                    span,
                    CommentGroup::default(),
                );

                Some(StringPart::Expression(ExpressionStringPart {
                    id: self.id(),
                    span: expression.span,
                    expression: Box::new(expression),
                }))
            }
            TokenKind::LeftBrace => {
                // "{$expr}"
                self.next();
                let e = self.parse_expression();
                self.skip_right_brace();
                Some(StringPart::Expression(ExpressionStringPart {
                    id: self.id(),
                    span: e.span,
                    expression: Box::new(e),
                }))
            }
            TokenKind::Variable => {
                // "$expr", "$expr[0]", "$expr[name]", "$expr->a"
                let variable_span = self.current_span();
                let variable = ExpressionKind::Variable(Box::new(self.parse_dynamic_variable()));
                let variable =
                    Expression::new(self.id(), variable, variable_span, CommentGroup::default());

                let e = match self.current_kind() {
                    TokenKind::LeftBracket => {
                        let left_bracket = self.skip_left_bracket();

                        // Full expression syntax is not allowed here,
                        // so we can't call expression.
                        let index = match self.current_kind() {
                            TokenKind::LiteralInteger => {
                                self.next();

                                ExpressionKind::Literal(Box::new(Literal::new(
                                    self.id(),
                                    LiteralKind::Integer,
                                    self.current().to_owned(),
                                    self.current_span(),
                                )))
                            }
                            TokenKind::Minus => {
                                self.next();

                                if let TokenKind::LiteralInteger = self.current_kind() {
                                    let span = self.current_span();
                                    let literal = self.current().to_owned();

                                    self.next();

                                    let kind = ExpressionKind::Literal(Box::new(Literal::new(
                                        self.id(),
                                        LiteralKind::Integer,
                                        literal,
                                        span,
                                    )));

                                    let expression = Expression::new(
                                        self.id(),
                                        kind,
                                        span,
                                        CommentGroup::default(),
                                    );

                                    ExpressionKind::ArithmeticOperation(Box::new(
                                        ArithmeticOperationExpression {
                                            id: self.id(),
                                            span: Span::combine(span, expression.span),
                                            kind: ArithmeticOperationKind::Negative {
                                                id: self.id(),
                                                minus: span,
                                                right: Box::new(expression),
                                            },
                                        },
                                    ))
                                } else {
                                    let span = self.current_span();

                                    self.diagnostic(
                                        ParserDiagnostic::ExpectedToken {
                                            expected: vec![TokenKind::LiteralInteger],
                                            found: self.current().to_owned(),
                                        },
                                        Severity::Error,
                                        span,
                                    );

                                    self.next();

                                    ExpressionKind::Missing(MissingExpression { id: 0, span })
                                }
                            }
                            TokenKind::Identifier => self.next_but_first(|parser| {
                                ExpressionKind::Literal(Box::new(Literal::new(
                                    parser.id(),
                                    LiteralKind::String,
                                    parser.current().to_owned(),
                                    parser.current_span(),
                                )))
                            }),
                            TokenKind::Variable => ExpressionKind::Variable(Box::new(
                                Variable::SimpleVariable(self.parse_simple_variable()),
                            )),
                            _ => {
                                let span = self.current_span();

                                self.diagnostic(
                                    ParserDiagnostic::ExpectedToken {
                                        expected: vec![
                                            TokenKind::LiteralInteger,
                                            TokenKind::Identifier,
                                            TokenKind::Variable,
                                        ],
                                        found: self.current().to_owned(),
                                    },
                                    Severity::Error,
                                    span,
                                );

                                self.next();

                                ExpressionKind::Missing(MissingExpression { id: 0, span })
                            }
                        };

                        let span = index.span();

                        let index =
                            Expression::new(self.id(), index, span, CommentGroup::default());

                        let right_bracket = self.skip_right_bracket();

                        ExpressionKind::ArrayIndex(Box::new(ArrayIndexExpression {
                            id: self.id(),
                            span: Span::combine(variable.span, right_bracket),
                            array: Box::new(variable),
                            left_bracket,
                            index: Some(Box::new(index)),
                            right_bracket,
                        }))
                    }
                    TokenKind::Arrow => {
                        let span = self.current_span();

                        self.next();

                        let identifier = self.parse_identifier_maybe_reserved();
                        let id_span = identifier.span;
                        let kind = ExpressionKind::Identifier(Box::new(
                            Identifier::SimpleIdentifier(identifier),
                        ));
                        let identifier_expression =
                            Expression::new(self.id(), kind, id_span, CommentGroup::default());

                        ExpressionKind::PropertyFetch(Box::new(PropertyFetchExpression {
                            id: self.id(),
                            span: Span::combine(variable.span, identifier_expression.span),
                            target: Box::new(variable),
                            arrow: span,
                            property: Box::new(identifier_expression),
                        }))
                    }
                    TokenKind::QuestionArrow => {
                        let span = self.next();
                        let ident = self.parse_identifier_maybe_reserved();
                        let ident_span = ident.span;
                        let kind = ExpressionKind::Identifier(Box::new(
                            Identifier::SimpleIdentifier(ident),
                        ));

                        ExpressionKind::NullsafePropertyFetch(Box::new(
                            NullsafePropertyFetchExpression {
                                id: self.id(),
                                span: Span::combine(variable.span, ident_span),
                                target: Box::new(variable),
                                question_arrow: span,
                                property: Box::new(Expression::new(
                                    self.id(),
                                    kind,
                                    ident_span,
                                    CommentGroup::default(),
                                )),
                            },
                        ))
                    }
                    _ => variable.kind.clone(),
                };

                let span = Span::combine(variable_span, e.span());

                Some(StringPart::Expression(ExpressionStringPart {
                    id: self.id(),
                    span,
                    expression: Box::new(Expression::new(
                        self.id(),
                        e,
                        span,
                        CommentGroup::default(),
                    )),
                }))
            }
            _ => {
                let span = self.current_span();

                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![
                            TokenKind::LeftBrace,
                            TokenKind::DollarLeftBrace,
                            TokenKind::DoubleQuote,
                            TokenKind::Variable,
                        ],
                        found: self.current().to_owned(),
                    },
                    Severity::Error,
                    span,
                );

                self.next();

                None
            }
        }
    }
}
