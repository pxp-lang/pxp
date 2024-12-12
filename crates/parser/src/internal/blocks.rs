use crate::internal::utils;
use crate::state::State;
use crate::statement;
use pxp_ast::BlockStatement;
use pxp_ast::Statement;
use pxp_ast::StatementKind;
use pxp_span::Span;
use pxp_token::OpenTagKind;
use pxp_token::TokenKind;

pub fn parse_block_statement(state: &mut State) -> StatementKind {
    let (left_brace, statements, right_brace) = utils::braced(state, &|state: &mut State| {
        parse_multiple_statements_until(state, &TokenKind::RightBrace)
    });

    StatementKind::Block(BlockStatement {
        id: state.id(),
        span: Span::combine(left_brace, right_brace),
        left_brace,
        statements,
        right_brace,
    })
}

pub fn parse_multiple_statements_until(state: &mut State, until: &TokenKind) -> Vec<Statement> {
    let mut statements = Vec::new();

    let mut current = state.current();
    while &current.kind != until {
        if let TokenKind::OpenTag(OpenTagKind::Full) = current.kind {
            state.next();

            current = state.current();
            continue;
        }

        statements.push(statement(state));
        current = state.current();
    }

    statements
}

pub fn parse_multiple_statements_until_any(state: &mut State, until: &[TokenKind]) -> Vec<Statement> {
    let mut statements = Vec::new();

    let mut current = state.current();
    while !until.contains(&current.kind) {
        if let TokenKind::OpenTag(OpenTagKind::Full) = current.kind {
            state.next();

            current = state.current();
            continue;
        }

        statements.push(statement(state));
        current = state.current();
    }

    statements
}
