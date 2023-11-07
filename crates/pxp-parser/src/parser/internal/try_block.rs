use crate::lexer::token::TokenKind;
use crate::parser::ast::try_block::CatchBlock;
use crate::parser::ast::try_block::CatchType;
use crate::parser::ast::try_block::FinallyBlock;
use crate::parser::ast::try_block::TryStatement;
use crate::parser::ast::Statement;
use crate::parser::error;
use crate::parser::error::ParseResult;
use crate::parser::internal::blocks;
use crate::parser::internal::identifiers;
use crate::parser::internal::utils;
use crate::parser::state::State;

use super::variables;

pub fn try_block(state: &mut State) -> ParseResult<Statement> {
    let start = state.stream.current().span;

    state.stream.next();
    utils::skip_left_brace(state)?;

    let body = blocks::multiple_statements_until(state, &TokenKind::RightBrace)?;

    let last_right_brace = utils::skip_right_brace(state)?;

    let mut catches = Vec::new();
    loop {
        if state.stream.current().kind != TokenKind::Catch {
            break;
        }

        let catch_start = state.stream.current().span;

        state.stream.next();
        utils::skip_left_parenthesis(state)?;

        let types = catch_type(state)?;
        let var = if state.stream.current().kind == TokenKind::RightParen {
            None
        } else {
            Some(variables::simple_variable(state)?)
        };

        utils::skip_right_parenthesis(state)?;
        utils::skip_left_brace(state)?;

        let catch_body = blocks::multiple_statements_until(state, &TokenKind::RightBrace)?;

        utils::skip_right_brace(state)?;

        let catch_end = state.stream.current().span;

        catches.push(CatchBlock {
            start: catch_start,
            end: catch_end,
            types,
            var,
            body: catch_body,
        })
    }

    let mut finally = None;
    if state.stream.current().kind == TokenKind::Finally {
        let finally_start = state.stream.current().span;
        state.stream.next();
        utils::skip_left_brace(state)?;

        let finally_body = blocks::multiple_statements_until(state, &TokenKind::RightBrace)?;

        utils::skip_right_brace(state)?;
        let finally_end = state.stream.current().span;

        finally = Some(FinallyBlock {
            start: finally_start,
            end: finally_end,
            body: finally_body,
        });
    }

    if catches.is_empty() && finally.is_none() {
        return Err(error::try_without_catch_or_finally(start, last_right_brace));
    }

    let end = state.stream.current().span;

    Ok(Statement::Try(TryStatement {
        start,
        end,
        body,
        catches,
        finally,
    }))
}

#[inline(always)]
fn catch_type(state: &mut State) -> ParseResult<CatchType> {
    let id = identifiers::full_name(state)?;

    if state.stream.current().kind == TokenKind::Pipe {
        state.stream.next();

        let mut types = vec![id];

        while !state.stream.is_eof() {
            let id = identifiers::full_name(state)?;
            types.push(id);

            if state.stream.current().kind != TokenKind::Pipe {
                break;
            }

            state.stream.next();
        }

        return Ok(CatchType::Union { identifiers: types });
    }

    Ok(CatchType::Identifier { identifier: id })
}
