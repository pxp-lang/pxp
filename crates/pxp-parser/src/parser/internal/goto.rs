use crate::lexer::token::TokenKind;
use crate::parser::ast::goto::GotoStatement;
use crate::parser::ast::goto::LabelStatement;
use crate::parser::ast::Statement;
use crate::parser::error::ParseResult;
use crate::parser::internal::identifiers;
use crate::parser::internal::utils;
use crate::parser::state::State;

pub fn label_statement(state: &mut State) -> ParseResult<Statement> {
    let comments = state.stream.comments();
    let label = identifiers::label_identifier(state)?;
    let colon = utils::skip_colon(state)?;

    Ok(Statement::Label(LabelStatement {
        comments,
        label,
        colon,
    }))
}

pub fn goto_statement(state: &mut State) -> ParseResult<Statement> {
    let comments = state.stream.comments();
    let keyword = utils::skip(state, TokenKind::Goto)?;
    let label = identifiers::label_identifier(state)?;
    let semicolon = utils::skip_semicolon(state)?;

    Ok(Statement::Goto(GotoStatement {
        comments,
        keyword,
        label,
        semicolon,
    }))
}
