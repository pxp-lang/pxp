use crate::expected_token_err;

use crate::lexer::token::TokenKind;
use crate::parser;
use crate::parser::ast::control_flow::IfStatement;
use crate::parser::ast::control_flow::IfStatementBody;
use crate::parser::ast::control_flow::IfStatementElse;
use crate::parser::ast::control_flow::IfStatementElseBlock;
use crate::parser::ast::control_flow::IfStatementElseIf;
use crate::parser::ast::control_flow::IfStatementElseIfBlock;
use crate::parser::ast::Case;
use crate::parser::ast::DefaultMatchArm;
use crate::parser::ast::Expression;
use crate::parser::ast::MatchArm;
use crate::parser::ast::Statement;
use crate::parser::ast::SwitchStatement;
use crate::parser::ast::{Block, MatchExpression};
use crate::parser::error;
use crate::parser::error::ParseResult;
use crate::parser::expressions;
use crate::parser::internal::blocks;
use crate::parser::internal::utils;
use crate::parser::state::State;

pub fn match_expression(state: &mut State) -> ParseResult<Expression> {
    let keyword = utils::skip(state, TokenKind::Match)?;

    let (left_parenthesis, condition, right_parenthesis) =
        utils::parenthesized(state, &|state: &mut State| {
            expressions::create(state).map(Box::new)
        })?;

    let left_brace = utils::skip_left_brace(state)?;

    let mut default: Option<Box<DefaultMatchArm>> = None;
    let mut arms = Vec::new();
    while state.stream.current().kind != TokenKind::RightBrace {
        let current = state.stream.current();
        if current.kind == TokenKind::Default {
            if let Some(default_arm) = default {
                return Err(error::match_expression_has_multiple_default_arms(
                    default_arm.keyword,
                    current.span,
                ));
            }

            state.stream.next();

            // match conditions can have an extra comma at the end, including `default`.
            if state.stream.current().kind == TokenKind::Comma {
                state.stream.next();
            }

            let arrow = utils::skip_double_arrow(state)?;

            let body = expressions::create(state)?;

            default = Some(Box::new(DefaultMatchArm {
                keyword: current.span,
                double_arrow: arrow,
                body,
            }));
        } else {
            let mut conditions = Vec::new();
            while state.stream.current().kind != TokenKind::DoubleArrow {
                conditions.push(expressions::create(state)?);

                if state.stream.current().kind == TokenKind::Comma {
                    state.stream.next();
                } else {
                    break;
                }
            }

            if conditions.is_empty() {
                break;
            }

            let arrow = utils::skip_double_arrow(state)?;

            let body = expressions::create(state)?;

            arms.push(MatchArm {
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

    let right_brace = utils::skip_right_brace(state)?;

    Ok(Expression::Match(MatchExpression {
        keyword,
        left_parenthesis,
        condition,
        right_parenthesis,
        left_brace,
        default,
        arms,
        right_brace,
    }))
}

pub fn switch_statement(state: &mut State) -> ParseResult<Statement> {
    let switch = utils::skip(state, TokenKind::Switch)?;

    let (left_parenthesis, condition, right_parenthesis) =
        utils::parenthesized(state, &expressions::create)?;

    let end_token = if state.stream.current().kind == TokenKind::Colon {
        utils::skip_colon(state)?;
        TokenKind::EndSwitch
    } else {
        utils::skip_left_brace(state)?;
        TokenKind::RightBrace
    };

    let mut cases = Vec::new();
    while state.stream.current().kind != end_token {
        match state.stream.current().kind {
            TokenKind::Case => {
                state.stream.next();

                let condition = expressions::create(state)?;

                utils::skip_any_of(state, &[TokenKind::Colon, TokenKind::SemiColon])?;

                let mut body = Block::new();

                while state.stream.current().kind != TokenKind::Case
                    && state.stream.current().kind != TokenKind::Default
                    && state.stream.current().kind != TokenKind::RightBrace
                    && state.stream.current().kind != end_token
                {
                    body.push(parser::statement(state)?);
                }

                cases.push(Case {
                    condition: Some(condition),
                    body,
                });
            }
            TokenKind::Default => {
                state.stream.next();

                utils::skip_any_of(state, &[TokenKind::Colon, TokenKind::SemiColon])?;

                let mut body = Block::new();

                while state.stream.current().kind != TokenKind::Case
                    && state.stream.current().kind != TokenKind::Default
                    && state.stream.current().kind != end_token
                {
                    body.push(parser::statement(state)?);
                }

                cases.push(Case {
                    condition: None,
                    body,
                });
            }
            _ => {
                return expected_token_err!(["`case`", "`default`"], state);
            }
        }
    }

    if end_token == TokenKind::EndSwitch {
        utils::skip(state, TokenKind::EndSwitch)?;
        utils::skip_ending(state)?;
    } else {
        utils::skip_right_brace(state)?;
    }

    Ok(Statement::Switch(SwitchStatement {
        switch,
        left_parenthesis,
        condition,
        right_parenthesis,
        cases,
    }))
}

pub fn if_statement(state: &mut State) -> ParseResult<Statement> {
    let r#if = utils::skip(state, TokenKind::If)?;

    let (left_parenthesis, condition, right_parenthesis) =
        utils::parenthesized(state, &expressions::create)?;

    Ok(Statement::If(IfStatement {
        r#if,
        left_parenthesis,
        condition,
        right_parenthesis,
        body: if state.stream.current().kind == TokenKind::Colon {
            if_statement_block_body(state)?
        } else {
            if_statement_statement_body(state)?
        },
    }))
}

fn if_statement_statement_body(state: &mut State) -> ParseResult<IfStatementBody> {
    let statement = parser::statement(state).map(Box::new)?;

    let mut elseifs: Vec<IfStatementElseIf> = vec![];
    let mut current = state.stream.current();
    while current.kind == TokenKind::ElseIf {
        state.stream.next();

        let (left_parenthesis, condition, right_parenthesis) =
            utils::parenthesized(state, &expressions::create)?;

        elseifs.push(IfStatementElseIf {
            elseif: current.span,
            left_parenthesis,
            condition,
            right_parenthesis,
            statement: parser::statement(state).map(Box::new)?,
        });

        current = state.stream.current();
    }

    let r#else = if current.kind == TokenKind::Else {
        state.stream.next();

        Some(IfStatementElse {
            r#else: current.span,
            statement: parser::statement(state).map(Box::new)?,
        })
    } else {
        None
    };

    Ok(IfStatementBody::Statement {
        statement,
        elseifs,
        r#else,
    })
}

fn if_statement_block_body(state: &mut State) -> ParseResult<IfStatementBody> {
    let colon = utils::skip(state, TokenKind::Colon)?;
    let statements = blocks::multiple_statements_until_any(
        state,
        &[TokenKind::Else, TokenKind::ElseIf, TokenKind::EndIf],
    )?;

    let mut elseifs: Vec<IfStatementElseIfBlock> = vec![];
    let mut current = state.stream.current();
    while current.kind == TokenKind::ElseIf {
        state.stream.next();

        let (left_parenthesis, condition, right_parenthesis) =
            utils::parenthesized(state, &expressions::create)?;

        elseifs.push(IfStatementElseIfBlock {
            elseif: current.span,
            left_parenthesis,
            condition,
            right_parenthesis,
            colon: utils::skip(state, TokenKind::Colon)?,
            statements: blocks::multiple_statements_until_any(
                state,
                &[TokenKind::Else, TokenKind::ElseIf, TokenKind::EndIf],
            )?,
        });

        current = state.stream.current();
    }

    let r#else = if current.kind == TokenKind::Else {
        state.stream.next();

        Some(IfStatementElseBlock {
            r#else: current.span,
            colon: utils::skip(state, TokenKind::Colon)?,
            statements: blocks::multiple_statements_until(state, &TokenKind::EndIf)?,
        })
    } else {
        None
    };

    Ok(IfStatementBody::Block {
        colon,
        statements,
        elseifs,
        r#else,
        endif: utils::skip(state, TokenKind::EndIf)?,
        ending: utils::skip_ending(state)?,
    })
}
