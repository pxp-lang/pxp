use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::parser;
use crate::parser::ast::literals::LiteralInteger;
use crate::parser::ast::loops::BreakStatement;
use crate::parser::ast::loops::ContinueStatement;
use crate::parser::ast::loops::DoWhileStatement;
use crate::parser::ast::loops::ForStatement;
use crate::parser::ast::loops::ForStatementBody;
use crate::parser::ast::loops::ForStatementIterator;
use crate::parser::ast::loops::ForeachStatement;
use crate::parser::ast::loops::ForeachStatementBody;
use crate::parser::ast::loops::ForeachStatementIterator;
use crate::parser::ast::loops::Level;
use crate::parser::ast::loops::WhileStatement;
use crate::parser::ast::loops::WhileStatementBody;
use crate::parser::ast::Statement;
use crate::parser::error::ParseResult;
use crate::parser::expressions;
use crate::parser::internal::blocks;
use crate::parser::internal::utils;
use crate::parser::state::State;

pub fn foreach_statement(state: &mut State) -> ParseResult<Statement> {
    let foreach = utils::skip(state, TokenKind::Foreach)?;

    let (left_parenthesis, iterator, right_parenthesis) =
        utils::parenthesized(state, &|state: &mut State| {
            let expression = expressions::create(state)?;

            let r#as = utils::skip(state, TokenKind::As)?;

            let current = state.stream.current();
            let ampersand = if current.kind == TokenKind::Ampersand {
                state.stream.next();
                Some(current.span)
            } else {
                None
            };

            let mut value = expressions::create(state)?;

            let current = state.stream.current();
            if current.kind == TokenKind::DoubleArrow {
                state.stream.next();
                let arrow = current.span;

                let current = state.stream.current();
                let ampersand = if current.kind == TokenKind::Ampersand {
                    state.stream.next();
                    Some(current.span)
                } else {
                    None
                };

                let mut key = expressions::create(state)?;

                std::mem::swap(&mut value, &mut key);

                Ok(ForeachStatementIterator::KeyAndValue {
                    expression,
                    r#as,
                    key,
                    double_arrow: arrow,
                    ampersand,
                    value,
                })
            } else {
                Ok(ForeachStatementIterator::Value {
                    expression,
                    r#as,
                    ampersand,
                    value,
                })
            }
        })?;

    let body = if state.stream.current().kind == TokenKind::Colon {
        ForeachStatementBody::Block {
            colon: utils::skip_colon(state)?,
            statements: blocks::multiple_statements_until(state, &TokenKind::EndForeach)?,
            endforeach: utils::skip(state, TokenKind::EndForeach)?,
            ending: utils::skip_ending(state)?,
        }
    } else {
        ForeachStatementBody::Statement {
            statement: parser::statement(state).map(Box::new)?,
        }
    };

    Ok(Statement::Foreach(ForeachStatement {
        foreach,
        left_parenthesis,
        iterator,
        right_parenthesis,
        body,
    }))
}

pub fn for_statement(state: &mut State) -> ParseResult<Statement> {
    let r#for = utils::skip(state, TokenKind::For)?;

    let (left_parenthesis, iterator, right_parenthesis) = utils::parenthesized(state, &|state| {
        let (initializations_semicolon, initializations) =
            utils::semicolon_terminated(state, &|state| {
                utils::comma_separated_no_trailing(
                    state,
                    &expressions::create,
                    TokenKind::SemiColon,
                )
            })?;

        let (conditions_semicolon, conditions) = utils::semicolon_terminated(state, &|state| {
            utils::comma_separated_no_trailing(state, &expressions::create, TokenKind::SemiColon)
        })?;

        Ok(ForStatementIterator {
            initializations,
            initializations_semicolon,
            conditions,
            conditions_semicolon,
            r#loop: utils::comma_separated_no_trailing(
                state,
                &expressions::create,
                TokenKind::RightParen,
            )?,
        })
    })?;

    let body = if state.stream.current().kind == TokenKind::Colon {
        ForStatementBody::Block {
            colon: utils::skip_colon(state)?,
            statements: blocks::multiple_statements_until(state, &TokenKind::EndFor)?,
            endfor: utils::skip(state, TokenKind::EndFor)?,
            ending: utils::skip_ending(state)?,
        }
    } else {
        ForStatementBody::Statement {
            statement: parser::statement(state).map(Box::new)?,
        }
    };

    Ok(Statement::For(ForStatement {
        r#for,
        left_parenthesis,
        iterator,
        right_parenthesis,
        body,
    }))
}

pub fn do_while_statement(state: &mut State) -> ParseResult<Statement> {
    let r#do = utils::skip(state, TokenKind::Do)?;

    let body = parser::statement(state).map(Box::new)?;

    let r#while = utils::skip(state, TokenKind::While)?;

    let (semicolon, (left_parenthesis, condition, right_parenthesis)) =
        utils::semicolon_terminated(state, &|state| {
            utils::parenthesized(state, &expressions::create)
        })?;

    Ok(Statement::DoWhile(DoWhileStatement {
        r#do,
        body,
        r#while,
        left_parenthesis,
        condition,
        right_parenthesis,
        semicolon,
    }))
}

pub fn while_statement(state: &mut State) -> ParseResult<Statement> {
    let r#while = utils::skip(state, TokenKind::While)?;

    let (left_parenthesis, condition, right_parenthesis) =
        utils::parenthesized(state, &expressions::create)?;

    let body = if state.stream.current().kind == TokenKind::Colon {
        WhileStatementBody::Block {
            colon: utils::skip_colon(state)?,
            statements: blocks::multiple_statements_until(state, &TokenKind::EndWhile)?,
            endwhile: utils::skip(state, TokenKind::EndWhile)?,
            ending: utils::skip_ending(state)?,
        }
    } else {
        WhileStatementBody::Statement {
            statement: parser::statement(state).map(Box::new)?,
        }
    };

    Ok(Statement::While(WhileStatement {
        r#while,
        left_parenthesis,
        condition,
        right_parenthesis,
        body,
    }))
}

pub fn continue_statement(state: &mut State) -> ParseResult<Statement> {
    Ok(Statement::Continue(ContinueStatement {
        r#continue: utils::skip(state, TokenKind::Continue)?,
        level: maybe_loop_level(state)?,
        ending: utils::skip_ending(state)?,
    }))
}

pub fn break_statement(state: &mut State) -> ParseResult<Statement> {
    Ok(Statement::Break(BreakStatement {
        r#break: utils::skip(state, TokenKind::Break)?,
        level: maybe_loop_level(state)?,
        ending: utils::skip_ending(state)?,
    }))
}

fn maybe_loop_level(state: &mut State) -> ParseResult<Option<Level>> {
    let current = &state.stream.current().kind;

    Ok(
        if current == &TokenKind::SemiColon || current == &TokenKind::CloseTag {
            None
        } else {
            Some(loop_level(state)?)
        },
    )
}

fn loop_level(state: &mut State) -> ParseResult<Level> {
    if let Token {
        kind: TokenKind::LiteralInteger,
        span,
        value,
    } = state.stream.current()
    {
        state.stream.next();

        return Ok(Level::Literal(LiteralInteger {
            value: value.clone(),
            span: *span,
        }));
    }

    let (left_parenthesis, level, right_parenthesis) =
        utils::parenthesized(state, &|state| loop_level(state).map(Box::new))?;

    Ok(Level::Parenthesized {
        left_parenthesis,
        level,
        right_parenthesis,
    })
}
