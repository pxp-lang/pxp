use crate::error::ParseResult;
use crate::internal::utils;
use crate::state::State;
use crate::statement;
use pxp_ast::BlockStatement;
use pxp_ast::Statement;
use pxp_ast::StatementKind;
use pxp_token::OpenTagKind;
use pxp_token::TokenKind;

pub fn block_statement(state: &mut State) -> ParseResult<StatementKind> {
    let (left_brace, statements, right_brace) = utils::braced(state, &|state: &mut State| {
        multiple_statements_until(state, &TokenKind::RightBrace)
    })?;

    Ok(StatementKind::Block(BlockStatement {
        left_brace,
        statements,
        right_brace,
    }))
}

pub fn multiple_statements_until(
    state: &mut State,
    until: &TokenKind,
) -> ParseResult<Vec<Statement>> {
    let mut statements = Vec::new();

    let mut current = state.stream.current();
    while &current.kind != until {
        if let TokenKind::OpenTag(OpenTagKind::Full) = current.kind {
            state.stream.next();

            current = state.stream.current();
            continue;
        }

        statements.push(statement(state)?);
        current = state.stream.current();
    }

    Ok(statements)
}

pub fn multiple_statements_until_any(
    state: &mut State,
    until: &[TokenKind],
) -> ParseResult<Vec<Statement>> {
    let mut statements = Vec::new();

    let mut current = state.stream.current();
    while !until.contains(&current.kind) {
        if let TokenKind::OpenTag(OpenTagKind::Full) = current.kind {
            state.stream.next();

            current = state.stream.current();
            continue;
        }

        statements.push(statement(state)?);
        current = state.stream.current();
    }

    Ok(statements)
}
