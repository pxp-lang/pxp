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
            self.parenthesized(|parser| Box::new(parser.parse_expression()));

        let left_brace = self.skip_left_brace();

        let mut default: Option<Box<DefaultMatchArm>> = None;
        let mut arms = Vec::new();
        while self.current_kind() != TokenKind::RightBrace {
            if self.current_kind() == TokenKind::Default {
                if default.is_some() {
                    self.diagnostic(
                        ParserDiagnostic::CannotHaveMultipleDefaultArmsInMatch,
                        Severity::Error,
                        self.current_span(),
                    );
                }

                let start = self.next();

                // match conditions can have an extra comma at the end, including `default`.
                if self.current_kind() == TokenKind::Comma {
                    self.next();
                }

                let arrow = self.skip_double_arrow();
                let body = self.parse_expression();

                default = Some(Box::new(DefaultMatchArm {
                    id: self.state.id(),
                    span: Span::combine(start, body.span),
                    keyword: start,
                    double_arrow: arrow,
                    body,
                }));
            } else {
                let mut conditions = Vec::new();

                while self.current_kind() != TokenKind::DoubleArrow {
                    conditions.push(self.parse_expression());

                    if self.current_kind() == TokenKind::Comma {
                        self.next();
                    } else {
                        break;
                    }
                }

                if conditions.is_empty() {
                    break;
                }

                let arrow = self.skip_double_arrow();

                let body = self.parse_expression();

                arms.push(MatchArm {
                    id: self.state.id(),
                    span: Span::combine(conditions.span(), body.span),
                    conditions,
                    arrow,
                    body,
                });
            }

            if self.current_kind() == TokenKind::Comma {
                self.next();
            } else {
                break;
            }
        }

        let right_brace = self.skip_right_brace();

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
            self.parenthesized(|parser| parser.parse_expression());

        let end_token = if self.current_kind() == TokenKind::Colon {
            self.skip_colon();
            TokenKind::EndSwitch
        } else {
            self.skip_left_brace();
            TokenKind::RightBrace
        };

        let mut cases = Vec::new();
        while self.current_kind() != end_token {
            match self.current_kind() {
                TokenKind::Case => {
                    self.next();

                    let condition = self.parse_expression();

                    self.skip_any_of(&[TokenKind::Colon, TokenKind::SemiColon]);

                    let mut body = Block::new();

                    while self.current_kind() != TokenKind::Case
                        && self.current_kind() != TokenKind::Default
                        && self.current_kind() != TokenKind::RightBrace
                        && self.current_kind() != end_token
                    {
                        body.push(self.parse_statement());
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

                    self.skip_any_of(&[TokenKind::Colon, TokenKind::SemiColon]);

                    let mut body = Block::new();

                    while self.current_kind() != TokenKind::Case
                        && self.current_kind() != TokenKind::Default
                        && self.current_kind() != end_token
                    {
                        body.push(self.parse_statement());
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
                            found: self.current().to_owned(),
                        },
                        Severity::Error,
                        self.current_span(),
                    );
                }
            }
        }

        if end_token == TokenKind::EndSwitch {
            self.skip(TokenKind::EndSwitch);
            self.skip_ending();
        } else {
            self.skip_right_brace();
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
            self.parenthesized(|parser| parser.parse_expression());

        let body = if self.current_kind() == TokenKind::Colon {
            self.parse_if_statement_block_body()
        } else {
            self.parse_if_statement_statement_body()
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
        let statement = Box::new(self.parse_statement());

        let mut elseifs: Vec<IfStatementElseIf> = vec![];

        while self.current_kind() == TokenKind::ElseIf {
            let start = self.next();

            let (left_parenthesis, condition, right_parenthesis) =
                self.parenthesized(|parser| parser.parse_expression());

            let statement = self.parse_statement();

            elseifs.push(IfStatementElseIf {
                id: self.state.id(),
                span: Span::combine(start, statement.span),
                elseif: start,
                left_parenthesis,
                condition,
                right_parenthesis,
                statement: Box::new(statement),
            });
        }

        let r#else = if self.current_kind() == TokenKind::Else {
            let start = self.next();

            let statement = self.parse_statement();

            Some(IfStatementElse {
                id: self.state.id(),
                span: Span::combine(start, statement.span),
                r#else: start,
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
        let statements = self.parse_multiple_statements_until_any(&[
            TokenKind::Else,
            TokenKind::ElseIf,
            TokenKind::EndIf,
        ]);

        let mut elseifs: Vec<IfStatementElseIfBlock> = vec![];

        while self.current_kind() == TokenKind::ElseIf {
            let start = self.next();

            let (left_parenthesis, condition, right_parenthesis) =
                self.parenthesized(|parser| parser.parse_expression());

            let colon = self.skip(TokenKind::Colon);

            let statements = self.parse_multiple_statements_until_any(&[
                TokenKind::Else,
                TokenKind::ElseIf,
                TokenKind::EndIf,
            ]);

            let span = Span::combine(start, statements.span());

            elseifs.push(IfStatementElseIfBlock {
                id: self.state.id(),
                span,
                elseif: start,
                left_parenthesis,
                condition,
                right_parenthesis,
                colon,
                statements,
            });
        }

        let r#else = if self.current_kind() == TokenKind::Else {
            let start = self.next();

            let colon = self.skip(TokenKind::Colon);
            let statements = self.parse_multiple_statements_until(TokenKind::EndIf);

            Some(Box::new(IfStatementElseBlock {
                id: self.state.id(),
                span: Span::combine(start, statements.span()),
                r#else: start,
                colon,
                statements,
            }))
        } else {
            None
        };

        let endif = self.skip(TokenKind::EndIf);
        let ending = self.skip_ending();

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
