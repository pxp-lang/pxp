use pxp_ast::Statement;
use pxp_token::TokenKind;

use crate::state::ParserState;

use super::{namespace, statement};

pub fn top_level_statement(state: &mut ParserState) -> Statement {
    match &state.stream.current().kind() {
        TokenKind::Namespace => namespace(state),
        _ => statement(state),
    }
}