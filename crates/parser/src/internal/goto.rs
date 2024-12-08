use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use crate::Parser;
use pxp_ast::StatementKind;
use pxp_ast::*;
use pxp_span::Span;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn label_statement(state: &mut State) -> StatementKind {
        let comments = state.comments();
        let label = identifiers::label_identifier(state);
        let colon = utils::skip_colon(state);

        StatementKind::Label(LabelStatement {
            id: state.id(),
            span: Span::combine(label.span, colon),
            comments,
            label,
            colon,
        })
    }

    pub fn goto_statement(state: &mut State) -> StatementKind {
        let comments = state.comments();
        let keyword = utils::skip(state, TokenKind::Goto);
        let label = identifiers::label_identifier(state);
        let semicolon = utils::skip_semicolon(state);

        StatementKind::Goto(GotoStatement {
            id: state.id(),
            span: Span::combine(keyword, semicolon),
            comments,
            keyword,
            label,
            semicolon,
        })
    }
}
