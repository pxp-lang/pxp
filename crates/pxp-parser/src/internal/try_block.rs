use crate::internal::blocks;
use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use crate::ParserDiagnostic;
use pxp_ast::*;
use pxp_ast::StatementKind;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

use super::variables;

pub fn try_block(state: &mut State) -> StatementKind {
    let start = state.stream.current().span;

    state.stream.next();
    utils::skip_left_brace(state);

    let body = blocks::multiple_statements_until(state, &TokenKind::RightBrace);

    let last_right_brace = utils::skip_right_brace(state);

    let mut catches = Vec::new();
    loop {
        if state.stream.current().kind != TokenKind::Catch {
            break;
        }

        let catch_start = state.stream.current().span;

        state.stream.next();
        utils::skip_left_parenthesis(state);

        let types = catch_type(state);
        let var = if state.stream.current().kind == TokenKind::RightParen {
            None
        } else {
            Some(variables::simple_variable(state))
        };

        utils::skip_right_parenthesis(state);
        utils::skip_left_brace(state);

        let catch_body = blocks::multiple_statements_until(state, &TokenKind::RightBrace);

        utils::skip_right_brace(state);

        let catch_end = state.stream.current().span;

        catches.push(CatchBlock {
            span: Span::combine(catch_start, catch_body.span()),
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
        utils::skip_left_brace(state);

        let finally_body = blocks::multiple_statements_until(state, &TokenKind::RightBrace);

        utils::skip_right_brace(state);
        let finally_end = state.stream.current().span;

        finally = Some(FinallyBlock {
            span: Span::combine(finally_start, finally_body.span()),
            start: finally_start,
            end: finally_end,
            body: finally_body,
        });
    }

    if catches.is_empty() && finally.is_none() {
        state.diagnostic(
            ParserDiagnostic::TryMustHaveCatchOrFinally,
            Severity::Error,
            last_right_brace,
        );
    }

    let end = state.stream.previous().span;

    StatementKind::Try(TryStatement {
        span: Span::combine(start, end),
        start,
        end,
        body,
        catches,
        finally,
    })
}

#[inline(always)]
fn catch_type(state: &mut State) -> CatchType {
    let id = identifiers::full_name(state);

    if state.stream.current().kind == TokenKind::Pipe {
        state.stream.next();

        let mut types = vec![id];

        while !state.stream.is_eof() {
            let id = identifiers::full_name(state);
            types.push(id);

            if state.stream.current().kind != TokenKind::Pipe {
                break;
            }

            state.stream.next();
        }

        return CatchType {
            span: types.span(),
            kind: CatchTypeKind::Union { identifiers: types }
        };
    }

    CatchType {
        span: id.span(),
        kind: CatchTypeKind::Identifier { identifier: id }
    }
}
