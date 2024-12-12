use crate::internal::blocks;
use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use crate::ParserDiagnostic;
use pxp_ast::StatementKind;
use pxp_ast::*;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

use super::variables;

pub fn parse_try_block(state: &mut State) -> StatementKind {
    let start = state.current().span;

    state.next();
    utils::skip_left_brace(state);

    let body = blocks::parse_multiple_statements_until(state, &TokenKind::RightBrace);

    let last_right_brace = utils::skip_right_brace(state);

    let mut catches = Vec::new();
    loop {
        if state.current().kind != TokenKind::Catch {
            break;
        }

        let catch_start = state.current().span;

        state.next();
        utils::skip_left_parenthesis(state);

        let types = parse_catch_type(state);
        let var = if state.current().kind == TokenKind::RightParen {
            None
        } else {
            Some(variables::parse_simple_variable(state))
        };

        utils::skip_right_parenthesis(state);
        utils::skip_left_brace(state);

        let catch_body = blocks::parse_multiple_statements_until(state, &TokenKind::RightBrace);

        utils::skip_right_brace(state);

        let catch_end = state.current().span;

        catches.push(CatchBlock {
            id: state.id(),
            span: Span::combine(catch_start, catch_body.span()),
            start: catch_start,
            end: catch_end,
            types,
            var,
            body: catch_body,
        })
    }

    let mut finally = None;
    if state.current().kind == TokenKind::Finally {
        let finally_start = state.current().span;
        state.next();
        utils::skip_left_brace(state);

        let finally_body = blocks::parse_multiple_statements_until(state, &TokenKind::RightBrace);

        utils::skip_right_brace(state);
        let finally_end = state.current().span;

        finally = Some(FinallyBlock {
            id: state.id(),
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

    let end = state.previous().span;

    StatementKind::Try(TryStatement {
        id: state.id(),
        span: Span::combine(start, end),
        start,
        end,
        body,
        catches,
        finally,
    })
}

#[inline(always)]
fn parse_catch_type(state: &mut State) -> CatchType {
    let id = identifiers::parse_full_name_identifier(state);

    if state.current().kind == TokenKind::Pipe {
        state.next();

        let mut types = vec![id];

        while !state.is_eof() {
            let id = identifiers::parse_full_name_identifier(state);
            types.push(id);

            if state.current().kind != TokenKind::Pipe {
                break;
            }

            state.next();
        }

        let span = types.span();

        return CatchType {
            id: state.id(),
            span,
            kind: CatchTypeKind::Union(CatchTypeKindUnion {
                id: state.id(),
                span,
                identifiers: types,
            }),
        };
    }

    CatchType {
        id: state.id(),
        span: id.span(),
        kind: CatchTypeKind::Identifier(CatchTypeKindIdentifier {
            id: state.id(),
            span: id.span(),
            identifier: id,
        }),
    }
}
