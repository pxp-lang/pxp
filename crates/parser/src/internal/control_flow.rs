use crate::expressions;
use crate::internal::blocks;
use crate::internal::utils;
use crate::state::State;
use crate::statement;
use crate::Parser;
use crate::ParserDiagnostic;
use pxp_ast::Case;
use pxp_ast::DefaultMatchArm;
use pxp_ast::Expression;
use pxp_ast::ExpressionKind;
use pxp_ast::MatchArm;
use pxp_ast::StatementKind;
use pxp_ast::SwitchStatement;
use pxp_ast::*;
use pxp_ast::{Block, MatchExpression};

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_match_expression(&mut self) -> Expression {
        let keyword = self.skip(TokenKind::Match);

        let (left_parenthesis, condition, right_parenthesis) =
            utils::parenthesized(state, &|&mut self| Box::new(self.parse_expression()));

        let left_brace = utils::skip_left_brace();

        let mut default: Option<Box<DefaultMatchArm>> = None;
        let mut arms = Vec::new();
        while self.current().kind != TokenKind::RightBrace {
            let current = self.current();
            if current.kind == TokenKind::Default {
                if default.is_some() {
                    self.diagnostic(
                        ParserDiagnostic::CannotHaveMultipleDefaultArmsInMatch,
                        Severity::Error,
                        current.span,
                    );
                }

                self.next();

                // match conditions can have an extra comma at the end, including `default`.
                if self.current().kind == TokenKind::Comma {
                    self.next();
                }

                let arrow = utils::skip_double_arrow();

                let body = self.parse_expression();

                default = Some(Box::new(DefaultMatchArm {
                    id: self.state.id(),
                    span: Span::combine(current.span, body.span),
                    keyword: current.span,
                    double_arrow: arrow,
                    body,
                }));
            } else {
                let mut conditions = Vec::new();

                while self.current().kind != TokenKind::DoubleArrow {
                    conditions.push(self.parse_expression());

                    if self.current().kind == TokenKind::Comma {
                        self.next();
                    } else {
                        break;
                    }
                }

                if conditions.is_empty() {
                    break;
                }

                let arrow = utils::skip_double_arrow();

                let body = self.parse_expression();

                arms.push(MatchArm {
                    id: self.state.id(),
                    span: Span::combine(conditions.span(), body.span),
                    conditions,
                    arrow,
                    body,
                });
            }

            if self.current().kind == TokenKind::Comma {
                self.next();
            } else {
                break;
            }
        }

        let right_brace = utils::skip_right_brace();

        Expression::new(
            self.state.id(),
            ExpressionKind::Match(MatchExpression {
                id: self.state.id(),
                span: Span::combine(keyword, right_brace),
                keyword,
                left_parenthesis,
                condition,
                right_parenthesis,
                left_brace,
                default,
                arms,
                right_brace,
            }),
            Span::combine(keyword, right_brace),
            CommentGroup::default(),
        )
    }

    pub fn parse_switch_statement(&mut self) -> StatementKind {
        let switch = self.skip(TokenKind::Switch);

        let (left_parenthesis, condition, right_parenthesis) =
            utils::parenthesized(state, &expressions::create);

        let end_token = if self.current().kind == TokenKind::Colon {
            utils::skip_colon();
            TokenKind::EndSwitch
        } else {
            utils::skip_left_brace();
            TokenKind::RightBrace
        };

        let mut cases = Vec::new();
        while self.current().kind != end_token {
            match self.current().kind {
                TokenKind::Case => {
                    self.next();

                    let condition = self.parse_expression();

                    utils::skip_any_of(state, &[TokenKind::Colon, TokenKind::SemiColon]);

                    let mut body = Block::new();

                    while self.current().kind != TokenKind::Case
                        && self.current().kind != TokenKind::Default
                        && self.current().kind != TokenKind::RightBrace
                        && self.current().kind != end_token
                    {
                        body.push(statement());
                    }

                    cases.push(Case {
                        id: self.state.id(),
                        span: Span::combine(condition.span, body.span()),
                        condition: Some(condition),
                        body,
                    });
                }
                TokenKind::Default => {
                    self.next();

                    utils::skip_any_of(state, &[TokenKind::Colon, TokenKind::SemiColon]);

                    let mut body = Block::new();

                    while self.current().kind != TokenKind::Case
                        && self.current().kind != TokenKind::Default
                        && self.current().kind != end_token
                    {
                        body.push(statement());
                    }

                    cases.push(Case {
                        id: self.state.id(),
                        span: body.span(),
                        condition: None,
                        body,
                    });
                }
                _ => {
                    self.diagnostic(
                        ParserDiagnostic::ExpectedToken {
                            expected: vec![TokenKind::Case, TokenKind::Default, end_token],
                            found: self.current().clone(),
                        },
                        Severity::Error,
                        self.current().span,
                    );
                }
            }
        }

        if end_token == TokenKind::EndSwitch {
            self.skip(TokenKind::EndSwitch);
            utils::skip_ending();
        } else {
            utils::skip_right_brace();
        }

        StatementKind::Switch(SwitchStatement {
            id: self.state.id(),
            span: Span::combine(switch, cases.span()),
            switch,
            left_parenthesis,
            condition,
            right_parenthesis,
            cases,
        })
    }

    pub fn parse_if_statement(&mut self) -> StatementKind {
        let r#if = self.skip(TokenKind::If);

        let (left_parenthesis, condition, right_parenthesis) =
            utils::parenthesized(state, &expressions::create);

        let body = if self.current().kind == TokenKind::Colon {
            parse_if_statement_block_body()
        } else {
            parse_if_statement_statement_body()
        };

        StatementKind::If(IfStatement {
            id: self.state.id(),
            span: Span::combine(r#if, body.span()),
            r#if,
            left_parenthesis,
            condition,
            right_parenthesis,
            body,
        })
    }

    fn parse_if_statement_statement_body(&mut self) -> IfStatementBody {
        let statement = Box::new(statement());

        let mut elseifs: Vec<IfStatementElseIf> = vec![];
        let mut current = self.current();
        while current.kind == TokenKind::ElseIf {
            self.next();

            let (left_parenthesis, condition, right_parenthesis) =
                utils::parenthesized(state, &expressions::create);

            let statement = crate::statement();

            elseifs.push(IfStatementElseIf {
                id: self.state.id(),
                span: Span::combine(current.span, statement.span),
                elseif: current.span,
                left_parenthesis,
                condition,
                right_parenthesis,
                statement: Box::new(statement),
            });

            current = self.current();
        }

        let r#else = if current.kind == TokenKind::Else {
            self.next();

            let statement = crate::statement();

            Some(IfStatementElse {
                id: self.state.id(),
                span: Span::combine(current.span, statement.span),
                r#else: current.span,
                statement: Box::new(statement),
            })
        } else {
            None
        };

        IfStatementBody::Statement(IfStatementBodyStatement {
            id: self.state.id(),
            span: if let Some(r#else) = &r#else {
                Span::combine(statement.span, r#else.span)
            } else {
                statement.span
            },
            statement,
            elseifs,
            r#else,
        })
    }

    fn parse_if_statement_block_body(&mut self) -> IfStatementBody {
        let colon = self.skip(TokenKind::Colon);
        let statements = blocks::parse_multiple_statements_until_any(
            state,
            &[TokenKind::Else, TokenKind::ElseIf, TokenKind::EndIf],
        );

        let mut elseifs: Vec<IfStatementElseIfBlock> = vec![];
        let mut current = self.current();
        while current.kind == TokenKind::ElseIf {
            self.next();

            let (left_parenthesis, condition, right_parenthesis) =
                utils::parenthesized(state, &expressions::create);

            let colon = self.skip(TokenKind::Colon);

            let statements = blocks::parse_multiple_statements_until_any(
                state,
                &[TokenKind::Else, TokenKind::ElseIf, TokenKind::EndIf],
            );

            let span = Span::combine(current.span, statements.span());

            elseifs.push(IfStatementElseIfBlock {
                id: self.state.id(),
                span,
                elseif: current.span,
                left_parenthesis,
                condition,
                right_parenthesis,
                colon,
                statements,
            });

            current = self.current();
        }

        let r#else = if current.kind == TokenKind::Else {
            self.next();

            let colon = self.skip(TokenKind::Colon);
            let statements = blocks::parse_multiple_statements_until(state, &TokenKind::EndIf);

            Some(IfStatementElseBlock {
                id: self.state.id(),
                span: Span::combine(current.span, statements.span()),
                r#else: current.span,
                colon,
                statements,
            })
        } else {
            None
        };

        let endif = self.skip(TokenKind::EndIf);
        let ending = utils::skip_ending();

        IfStatementBody::Block(IfStatementBodyBlock {
            id: self.state.id(),
            span: Span::combine(colon, ending.span()),
            colon,
            statements,
            elseifs,
            r#else,
            endif,
            ending,
        })
    }
}
