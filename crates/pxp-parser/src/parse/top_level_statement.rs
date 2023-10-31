use pxp_ast::{Statement, StatementKind, InlineHtmlStatement, HaltCompilerStatement};
use pxp_span::Span;
use pxp_token::TokenKind;

use crate::state::ParserState;

use super::{namespace, statement, r#use, r#const};

pub fn top_level_statement(state: &mut ParserState) -> Statement {
    match &state.stream.current().kind() {
        TokenKind::Namespace => namespace(state),
        TokenKind::Use => r#use(state),
        TokenKind::Const => r#const(state),
        TokenKind::HaltCompiler => {
            let halt_compiler = state.stream.current().clone();
            state.stream.next();

            let content = if let TokenKind::InlineHtml = state.stream.current().kind {
                let content = state.stream.current().clone();
                state.stream.next();
                Some(content)
            } else {
                None
            };

            Statement::new(
                StatementKind::HaltCompiler(HaltCompilerStatement { content }),
                Span::new(halt_compiler.span.start, state.stream.previous().span.end)
            )
        },
        _ => statement(state),
    }
}