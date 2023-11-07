use crate::lexer::token::OpenTagKind;
use crate::lexer::token::TokenKind;
use crate::parser;
use crate::parser::ast::BlockStatement;
use crate::parser::ast::Statement;
use crate::parser::error::ParseResult;
use crate::parser::internal::utils;
use crate::parser::state::State;

pub fn block_statement(state: &mut State) -> ParseResult<Statement> {
    let (left_brace, statements, right_brace) = utils::braced(state, &|state: &mut State| {
        multiple_statements_until(state, &TokenKind::RightBrace)
    })?;

    Ok(Statement::Block(BlockStatement {
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

        statements.push(parser::statement(state)?);
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

        statements.push(parser::statement(state)?);
        current = state.stream.current();
    }

    Ok(statements)
}
