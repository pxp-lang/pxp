use crate::expressions;
use crate::internal::blocks;
use crate::internal::utils;
use crate::state::State;
use crate::statement;
use crate::ParserDiagnostic;
use pxp_ast::*;
use pxp_ast::Case;
use pxp_ast::DefaultMatchArm;
use pxp_ast::Expression;
use pxp_ast::ExpressionKind;
use pxp_ast::MatchArm;
use pxp_ast::StatementKind;
use pxp_ast::SwitchStatement;
use pxp_ast::{Block, MatchExpression};

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_syntax::comments::CommentGroup;
use pxp_token::TokenKind;

pub fn match_expression(state: &mut State) -> Expression {
    let keyword = utils::skip(state, TokenKind::Match);

    let (left_parenthesis, condition, right_parenthesis) =
        utils::parenthesized(state, &|state: &mut State| {
            Box::new(expressions::create(state))
        });

    let left_brace = utils::skip_left_brace(state);

    let mut default: Option<Box<DefaultMatchArm>> = None;
    let mut arms = Vec::new();
    while state.stream.current().kind != TokenKind::RightBrace {
        let current = state.stream.current();
        if current.kind == TokenKind::Default {
            if default.is_some() {
                state.diagnostic(
                    ParserDiagnostic::CannotHaveMultipleDefaultArmsInMatch,
                    Severity::Error,
                    current.span,
                );
            }

            state.stream.next();

            // match conditions can have an extra comma at the end, including `default`.
            if state.stream.current().kind == TokenKind::Comma {
                state.stream.next();
            }

            let arrow = utils::skip_double_arrow(state);

            let body = expressions::create(state);

            default = Some(Box::new(DefaultMatchArm {
                 id: state.id(), 
                span: Span::combine(current.span, body.span),
                keyword: current.span,
                double_arrow: arrow,
                body,
            }));
        } else {
            let mut conditions = Vec::new();

            while state.stream.current().kind != TokenKind::DoubleArrow {
                conditions.push(expressions::create(state));

                if state.stream.current().kind == TokenKind::Comma {
                    state.stream.next();
                } else {
                    break;
                }
            }

            if conditions.is_empty() {
                break;
            }

            let arrow = utils::skip_double_arrow(state);

            let body = expressions::create(state);

            arms.push(MatchArm {
                 id: state.id(), 
                span: Span::combine(conditions.span(), body.span),
                conditions,
                arrow,
                body,
            });
        }

        if state.stream.current().kind == TokenKind::Comma {
            state.stream.next();
        } else {
            break;
        }
    }

    let right_brace = utils::skip_right_brace(state);

    Expression::new(
            state.id(),
        ExpressionKind::Match(MatchExpression {
             id: state.id(), 
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

pub fn switch_statement(state: &mut State) -> StatementKind {
    let switch = utils::skip(state, TokenKind::Switch);

    let (left_parenthesis, condition, right_parenthesis) =
        utils::parenthesized(state, &expressions::create);

    let end_token = if state.stream.current().kind == TokenKind::Colon {
        utils::skip_colon(state);
        TokenKind::EndSwitch
    } else {
        utils::skip_left_brace(state);
        TokenKind::RightBrace
    };

    let mut cases = Vec::new();
    while state.stream.current().kind != end_token {
        match state.stream.current().kind {
            TokenKind::Case => {
                state.stream.next();

                let condition = expressions::create(state);

                utils::skip_any_of(state, &[TokenKind::Colon, TokenKind::SemiColon]);

                let mut body = Block::new();

                while state.stream.current().kind != TokenKind::Case
                    && state.stream.current().kind != TokenKind::Default
                    && state.stream.current().kind != TokenKind::RightBrace
                    && state.stream.current().kind != end_token
                {
                    body.push(statement(state));
                }

                cases.push(Case {
                     id: state.id(), 
                    span: Span::combine(condition.span, body.span()),
                    condition: Some(condition),
                    body,
                });
            }
            TokenKind::Default => {
                state.stream.next();

                utils::skip_any_of(state, &[TokenKind::Colon, TokenKind::SemiColon]);

                let mut body = Block::new();

                while state.stream.current().kind != TokenKind::Case
                    && state.stream.current().kind != TokenKind::Default
                    && state.stream.current().kind != end_token
                {
                    body.push(statement(state));
                }

                cases.push(Case {
                     id: state.id(), 
                    span: body.span(),
                    condition: None,
                    body,
                });
            }
            _ => {
                state.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Case, TokenKind::Default, end_token],
                        found: *state.stream.current(),
                    },
                    Severity::Error,
                    state.stream.current().span,
                );
            }
        }
    }

    if end_token == TokenKind::EndSwitch {
        utils::skip(state, TokenKind::EndSwitch);
        utils::skip_ending(state);
    } else {
        utils::skip_right_brace(state);
    }

    StatementKind::Switch(SwitchStatement {
         id: state.id(), 
        span: Span::combine(switch, cases.span()),
        switch,
        left_parenthesis,
        condition,
        right_parenthesis,
        cases,
    })
}

pub fn if_statement(state: &mut State) -> StatementKind {
    let r#if = utils::skip(state, TokenKind::If);

    let (left_parenthesis, condition, right_parenthesis) =
        utils::parenthesized(state, &expressions::create);

    let body = if state.stream.current().kind == TokenKind::Colon {
        if_statement_block_body(state)
    } else {
        if_statement_statement_body(state)
    };

    StatementKind::If(IfStatement {
         id: state.id(), 
        span: Span::combine(r#if, body.span()),
        r#if,
        left_parenthesis,
        condition,
        right_parenthesis,
        body,
    })
}

fn if_statement_statement_body(state: &mut State) -> IfStatementBody {
    let statement = Box::new(statement(state));

    let mut elseifs: Vec<IfStatementElseIf> = vec![];
    let mut current = state.stream.current();
    while current.kind == TokenKind::ElseIf {
        state.stream.next();

        let (left_parenthesis, condition, right_parenthesis) =
            utils::parenthesized(state, &expressions::create);

        let statement = crate::statement(state);

        elseifs.push(IfStatementElseIf {
             id: state.id(), 
            span: Span::combine(current.span, statement.span),
            elseif: current.span,
            left_parenthesis,
            condition,
            right_parenthesis,
            statement: Box::new(statement),
        });

        current = state.stream.current();
    }

    let r#else = if current.kind == TokenKind::Else {
        state.stream.next();

        let statement = crate::statement(state);

        Some(IfStatementElse {
             id: state.id(), 
            span: Span::combine(current.span, statement.span),
            r#else: current.span,
            statement: Box::new(statement),
        })
    } else {
        None
    };

    IfStatementBody::Statement(IfStatementBodyStatement {
        id: state.id(),
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

fn if_statement_block_body(state: &mut State) -> IfStatementBody {
    let colon = utils::skip(state, TokenKind::Colon);
    let statements = blocks::multiple_statements_until_any(
        state,
        &[TokenKind::Else, TokenKind::ElseIf, TokenKind::EndIf],
    );

    let mut elseifs: Vec<IfStatementElseIfBlock> = vec![];
    let mut current = state.stream.current();
    while current.kind == TokenKind::ElseIf {
        state.stream.next();

        let (left_parenthesis, condition, right_parenthesis) =
            utils::parenthesized(state, &expressions::create);

        let colon = utils::skip(state, TokenKind::Colon);

        let statements = blocks::multiple_statements_until_any(
            state,
            &[TokenKind::Else, TokenKind::ElseIf, TokenKind::EndIf],
        );

        let span = Span::combine(current.span, statements.span());

        elseifs.push(IfStatementElseIfBlock {
             id: state.id(), 
            span,
            elseif: current.span,
            left_parenthesis,
            condition,
            right_parenthesis,
            colon,
            statements,
        });

        current = state.stream.current();
    }

    let r#else = if current.kind == TokenKind::Else {
        state.stream.next();

        let colon = utils::skip(state, TokenKind::Colon);
        let statements = blocks::multiple_statements_until(state, &TokenKind::EndIf);

        Some(IfStatementElseBlock {
             id: state.id(), 
            span: Span::combine(current.span, statements.span()),
            r#else: current.span,
            colon,
            statements,
        })
    } else {
        None
    };

    let endif = utils::skip(state, TokenKind::EndIf);
    let ending = utils::skip_ending(state);

    IfStatementBody::Block(IfStatementBodyBlock {
        id: state.id(),
        span: Span::combine(colon, ending.span()),
        colon,
        statements,
        elseifs,
        r#else,
        endif,
        ending,
    })
}
