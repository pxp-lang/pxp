use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use pxp_ast::goto::GotoStatement;
use pxp_ast::goto::LabelStatement;
use pxp_ast::StatementKind;
use pxp_token::TokenKind;

pub fn label_statement(state: &mut State) -> StatementKind {
    let comments = state.stream.comments();
    let label = identifiers::label_identifier(state);
    let colon = utils::skip_colon(state);

    StatementKind::Label(LabelStatement {
        comments,
        label,
        colon,
    })
}

pub fn goto_statement(state: &mut State) -> StatementKind {
    let comments = state.stream.comments();
    let keyword = utils::skip(state, TokenKind::Goto);
    let label = identifiers::label_identifier(state);
    let semicolon = utils::skip_semicolon(state);

    StatementKind::Goto(GotoStatement {
        comments,
        keyword,
        label,
        semicolon,
    })
}
