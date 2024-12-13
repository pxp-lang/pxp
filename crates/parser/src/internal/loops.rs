use crate::expressions;
use crate::internal::blocks;
use crate::internal::utils;
use crate::state::State;
use crate::statement;
use crate::Parser;
use pxp_ast::StatementKind;
use pxp_ast::*;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::Token;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_foreach_statement(&mut self) -> StatementKind {
        let foreach = self.skip(TokenKind::Foreach);

        let (left_parenthesis, iterator, right_parenthesis) =
            self.parenthesized(&|&mut self| {
                let expression = self.parse_expression();

                let r#as = self.skip(TokenKind::As);

                let current = self.current();
                let ampersand = if current.kind == TokenKind::Ampersand {
                    self.next();
                    Some(current.span)
                } else {
                    None
                };

                let mut value = self.parse_expression();

                let current = self.current();
                if current.kind == TokenKind::DoubleArrow {
                    self.next();
                    let arrow = current.span;

                    let current = self.current();
                    let ampersand = if current.kind == TokenKind::Ampersand {
                        self.next();
                        Some(current.span)
                    } else {
                        None
                    };

                    let mut key = self.parse_expression();

                    std::mem::swap(&mut value, &mut key);

                    ForeachStatementIterator::KeyAndValue(ForeachStatementIteratorKeyAndValue {
                        id: self.state.id(),
                        span: Span::combine(expression.span, value.span),
                        expression,
                        r#as,
                        key,
                        double_arrow: arrow,
                        ampersand,
                        value,
                    })
                } else {
                    ForeachStatementIterator::Value(ForeachStatementIteratorValue {
                        id: self.state.id(),
                        span: Span::combine(expression.span, value.span),
                        expression,
                        r#as,
                        ampersand,
                        value,
                    })
                }
            });

        let body = if self.current_kind() == TokenKind::Colon {
            let colon = self.skip_colon();
            let statements = self.parse_multiple_statements_until(TokenKind::EndForeach);
            let endforeach = self.skip(TokenKind::EndForeach);
            let ending = self.skip_ending();

            ForeachStatementBody::Block(ForeachStatementBodyBlock {
                id: self.state.id(),
                span: Span::combine(colon, ending.span()),
                colon,
                statements,
                endforeach,
                ending,
            })
        } else {
            let statement = statement();

            ForeachStatementBody::Statement(ForeachStatementBodyStatement {
                id: self.state.id(),
                span: statement.span,
                statement: Box::new(statement),
            })
        };

        StatementKind::Foreach(ForeachStatement {
            id: self.state.id(),
            span: Span::combine(foreach, body.span()),
            foreach,
            left_parenthesis,
            iterator,
            right_parenthesis,
            body,
        })
    }

    pub fn parse_for_statement(&mut self) -> StatementKind {
        let r#for = self.skip(TokenKind::For);

        let (left_parenthesis, iterator, right_parenthesis) =
            self.parenthesized(&|state| {
                let (initializations_semicolon, initializations) =
                    self.semicolon_terminated(&|state| {
                        self.comma_separated_no_trailing(
                            state,
                            &expressions::create,
                            TokenKind::SemiColon,
                        )
                    });

                let (conditions_semicolon, conditions) =
                    self.semicolon_terminated(&|state| {
                        self.comma_separated_no_trailing(
                            state,
                            &expressions::create,
                            TokenKind::SemiColon,
                        )
                    });

                let r#loop = self.comma_separated_no_trailing(
                    state,
                    &expressions::create,
                    TokenKind::RightParen,
                );

                ForStatementIterator {
                    id: self.state.id(),
                    span: Span::combine(initializations.span(), r#loop.span()),
                    initializations,
                    initializations_semicolon,
                    conditions,
                    conditions_semicolon,
                    r#loop,
                }
            });

        let body = if self.current_kind() == TokenKind::Colon {
            let colon = self.skip_colon();
            let statements = self.parse_multiple_statements_until(TokenKind::EndFor);
            let endfor = self.skip(TokenKind::EndFor);
            let ending = self.skip_ending();

            ForStatementBody::Block(ForStatementBodyBlock {
                id: self.state.id(),
                span: Span::combine(colon, ending.span()),
                colon,
                statements,
                endfor,
                ending,
            })
        } else {
            let x = statement();

            ForStatementBody::Statement(ForStatementBodyStatement {
                id: self.state.id(),
                span: x.span,
                statement: Box::new(x),
            })
        };

        StatementKind::For(ForStatement {
            id: self.state.id(),
            span: Span::combine(r#for, body.span()),
            r#for,
            left_parenthesis,
            iterator,
            right_parenthesis,
            body,
        })
    }

    pub fn parse_do_while_statement(&mut self) -> StatementKind {
        let r#do = self.skip(TokenKind::Do);

        let body = Box::new(statement());

        let r#while = self.skip(TokenKind::While);

        let (semicolon, (left_parenthesis, condition, right_parenthesis)) =
            self.semicolon_terminated(&|state| {
                self.parenthesized(|parser| parser.parse_expression())
            });

        StatementKind::DoWhile(DoWhileStatement {
            id: self.state.id(),
            span: Span::combine(r#do, right_parenthesis),
            r#do,
            body,
            r#while,
            left_parenthesis,
            condition,
            right_parenthesis,
            semicolon,
        })
    }

    pub fn parse_while_statement(&mut self) -> StatementKind {
        let r#while = self.skip(TokenKind::While);

        let (left_parenthesis, condition, right_parenthesis) =
            self.parenthesized(|parser| parser.parse_expression());

        let body = if self.current_kind() == TokenKind::Colon {
            let colon = self.skip_colon();
            let statements = self.parse_multiple_statements_until(TokenKind::EndWhile);
            let endwhile = self.skip(TokenKind::EndWhile);
            let ending = self.skip_ending();

            WhileStatementBody::Block(WhileStatementBodyBlock {
                id: self.state.id(),
                span: Span::combine(colon, ending.span()),
                colon,
                statements,
                endwhile,
                ending,
            })
        } else {
            let x = statement();

            WhileStatementBody::Statement(WhileStatementBodyStatement {
                id: self.state.id(),
                span: x.span,
                statement: Box::new(x),
            })
        };

        StatementKind::While(WhileStatement {
            id: self.state.id(),
            span: Span::combine(r#while, body.span()),
            r#while,
            left_parenthesis,
            condition,
            right_parenthesis,
            body,
        })
    }

    pub fn parse_continue_statement(&mut self) -> StatementKind {
        let r#continue = self.skip(TokenKind::Continue);
        let level = maybe_parse_loop_level();
        let ending = self.skip_ending();

        StatementKind::Continue(ContinueStatement {
            id: self.state.id(),
            span: Span::combine(r#continue, ending.span()),
            r#continue,
            level,
            ending,
        })
    }

    pub fn parse_break_statement(&mut self) -> StatementKind {
        let r#break = self.skip(TokenKind::Break);
        let level = maybe_parse_loop_level();
        let ending = self.skip_ending();

        StatementKind::Break(BreakStatement {
            id: self.state.id(),
            span: Span::combine(r#break, ending.span()),
            r#break,
            level,
            ending,
        })
    }

    fn maybe_parse_loop_level(&mut self) -> Option<Level> {
        let current = &self.current_kind();

        if current == &TokenKind::SemiColon || current == &TokenKind::CloseTag {
            None
        } else {
            Some(parse_loop_level())
        }
    }

    fn parse_loop_level(&mut self) -> Level {
        let current = self.current();

        if let Token {
            kind: TokenKind::LiteralInteger,
            ..
        } = current
        {
            self.next();

            return Level::Literal(LiteralLevel {
                id: self.state.id(),
                literal: Literal::new(
                    self.state.id(),
                    LiteralKind::Integer,
                    current.clone(),
                    current.span,
                ),
            });
        }

        let (left_parenthesis, level, right_parenthesis) =
            self.parenthesized(&|state| Box::new(parse_loop_level()));

        Level::Parenthesized(ParenthesizedLevel {
            id: self.state.id(),
            span: Span::combine(left_parenthesis, right_parenthesis),
            left_parenthesis,
            level,
            right_parenthesis,
        })
    }
}
