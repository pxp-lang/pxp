use crate::Parser;
use pxp_ast::StatementKind;
use pxp_ast::*;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_foreach_statement(&mut self) -> StatementKind {
        let foreach = self.skip(TokenKind::Foreach);

        let (left_parenthesis, iterator, right_parenthesis) = self.parenthesized(|parser| {
            let expression = parser.parse_expression();

            let r#as = parser.skip(TokenKind::As);

            let ampersand = if parser.current_kind() == TokenKind::Ampersand {
                Some(parser.next())
            } else {
                None
            };

            let mut value = parser.parse_expression();

            if parser.current_kind() == TokenKind::DoubleArrow {
                let arrow = parser.next();

                let ampersand = if parser.current_kind() == TokenKind::Ampersand {
                    Some(parser.next())
                } else {
                    None
                };

                let mut key = parser.parse_expression();

                std::mem::swap(&mut value, &mut key);

                ForeachStatementIterator::KeyAndValue(ForeachStatementIteratorKeyAndValue {
                    id: parser.id(),
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
                    id: parser.id(),
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
                id: self.id(),
                span: Span::combine(colon, ending.span()),
                colon,
                statements,
                endforeach,
                ending,
            })
        } else {
            let statement = self.parse_statement();

            ForeachStatementBody::Statement(ForeachStatementBodyStatement {
                id: self.id(),
                span: statement.span,
                statement: Box::new(statement),
            })
        };

        StatementKind::Foreach(Box::new(ForeachStatement {
            id: self.id(),
            span: Span::combine(foreach, body.span()),
            foreach,
            left_parenthesis,
            iterator,
            right_parenthesis,
            body,
        }))
    }

    pub fn parse_for_statement(&mut self) -> StatementKind {
        let r#for = self.skip(TokenKind::For);

        let (left_parenthesis, iterator, right_parenthesis) = self.parenthesized(|parser| {
            let (initializations_semicolon, initializations) =
                parser.semicolon_terminated(|parser| {
                    parser.comma_separated_no_trailing(
                        |parser| parser.parse_expression(),
                        TokenKind::SemiColon,
                    )
                });

            let (conditions_semicolon, conditions) = parser.semicolon_terminated(|parser| {
                parser.comma_separated_no_trailing(
                    |parser| parser.parse_expression(),
                    TokenKind::SemiColon,
                )
            });

            let r#loop = parser.comma_separated_no_trailing(
                |parser| parser.parse_expression(),
                TokenKind::RightParen,
            );

            ForStatementIterator {
                id: parser.id(),
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
                id: self.id(),
                span: Span::combine(colon, ending.span()),
                colon,
                statements,
                endfor,
                ending,
            })
        } else {
            let x = self.parse_statement();

            ForStatementBody::Statement(ForStatementBodyStatement {
                id: self.id(),
                span: x.span,
                statement: Box::new(x),
            })
        };

        StatementKind::For(Box::new(ForStatement {
            id: self.id(),
            span: Span::combine(r#for, body.span()),
            r#for,
            left_parenthesis,
            iterator,
            right_parenthesis,
            body,
        }))
    }

    pub fn parse_do_while_statement(&mut self) -> StatementKind {
        let r#do = self.skip(TokenKind::Do);

        let body = Box::new(self.parse_statement());

        let r#while = self.skip(TokenKind::While);

        let (semicolon, (left_parenthesis, condition, right_parenthesis)) = self
            .semicolon_terminated(|parser| {
                parser.parenthesized(|parser| parser.parse_expression())
            });

        StatementKind::DoWhile(Box::new(DoWhileStatement {
            id: self.id(),
            span: Span::combine(r#do, right_parenthesis),
            r#do,
            body,
            r#while,
            left_parenthesis,
            condition,
            right_parenthesis,
            semicolon,
        }))
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
                id: self.id(),
                span: Span::combine(colon, ending.span()),
                colon,
                statements,
                endwhile,
                ending,
            })
        } else {
            let x = self.parse_statement();

            WhileStatementBody::Statement(WhileStatementBodyStatement {
                id: self.id(),
                span: x.span,
                statement: Box::new(x),
            })
        };

        StatementKind::While(Box::new(WhileStatement {
            id: self.id(),
            span: Span::combine(r#while, body.span()),
            r#while,
            left_parenthesis,
            condition,
            right_parenthesis,
            body,
        }))
    }

    pub fn parse_continue_statement(&mut self) -> StatementKind {
        let r#continue = self.skip(TokenKind::Continue);
        let level = self.maybe_parse_loop_level();
        let ending = self.skip_ending();

        StatementKind::Continue(Box::new(ContinueStatement {
            id: self.id(),
            span: Span::combine(r#continue, ending.span()),
            r#continue,
            level,
            ending,
        }))
    }

    pub fn parse_break_statement(&mut self) -> StatementKind {
        let r#break = self.skip(TokenKind::Break);
        let level = self.maybe_parse_loop_level();
        let ending = self.skip_ending();

        StatementKind::Break(Box::new(BreakStatement {
            id: self.id(),
            span: Span::combine(r#break, ending.span()),
            r#break,
            level,
            ending,
        }))
    }

    fn maybe_parse_loop_level(&mut self) -> Option<Level> {
        let current = &self.current_kind();

        if current == &TokenKind::SemiColon || current == &TokenKind::CloseTag {
            None
        } else {
            Some(self.parse_loop_level())
        }
    }

    fn parse_loop_level(&mut self) -> Level {
        if self.current_kind() == TokenKind::LiteralInteger {
            let token = self.current().to_owned();
            let span = token.span;
            self.next();

            return Level::Literal(LiteralLevel {
                id: self.id(),
                literal: Literal::new(self.id(), LiteralKind::Integer, token, span),
            });
        }

        let (left_parenthesis, level, right_parenthesis) =
            self.parenthesized(|parser| Box::new(parser.parse_loop_level()));

        Level::Parenthesized(ParenthesizedLevel {
            id: self.id(),
            span: Span::combine(left_parenthesis, right_parenthesis),
            left_parenthesis,
            level,
            right_parenthesis,
        })
    }
}
