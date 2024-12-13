use crate::internal::blocks;
use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use crate::Parser;
use crate::ParserDiagnostic;
use pxp_ast::StatementKind;
use pxp_ast::*;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

use super::variables;

impl<'a> Parser<'a> {
    pub fn parse_try_block(&mut self) -> StatementKind {
        let start = self.current().span;

        self.next();
        utils::skip_left_brace();

        let body = blocks::parse_multiple_statements_until(state, &TokenKind::RightBrace);

        let last_right_brace = utils::skip_right_brace();

        let mut catches = Vec::new();
        loop {
            if self.current().kind != TokenKind::Catch {
                break;
            }

            let catch_start = self.current().span;

            self.next();
            self.skip_left_parenthesis();

            let types = parse_catch_type();
            let var = if self.current().kind == TokenKind::RightParen {
                None
            } else {
                Some(variables::parse_simple_variable())
            };

            self.skip_right_parenthesis();
            utils::skip_left_brace();

            let catch_body = blocks::parse_multiple_statements_until(state, &TokenKind::RightBrace);

            utils::skip_right_brace();

            let catch_end = self.current().span;

            catches.push(CatchBlock {
                id: self.state.id(),
                span: Span::combine(catch_start, catch_body.span()),
                start: catch_start,
                end: catch_end,
                types,
                var,
                body: catch_body,
            })
        }

        let mut finally = None;
        if self.current().kind == TokenKind::Finally {
            let finally_start = self.current().span;
            self.next();
            utils::skip_left_brace();

            let finally_body =
                blocks::parse_multiple_statements_until(state, &TokenKind::RightBrace);

            utils::skip_right_brace();
            let finally_end = self.current().span;

            finally = Some(FinallyBlock {
                id: self.state.id(),
                span: Span::combine(finally_start, finally_body.span()),
                start: finally_start,
                end: finally_end,
                body: finally_body,
            });
        }

        if catches.is_empty() && finally.is_none() {
            self.diagnostic(
                ParserDiagnostic::TryMustHaveCatchOrFinally,
                Severity::Error,
                last_right_brace,
            );
        }

        let end = state.previous().span;

        StatementKind::Try(TryStatement {
            id: self.state.id(),
            span: Span::combine(start, end),
            start,
            end,
            body,
            catches,
            finally,
        })
    }

    #[inline(always)]
    fn parse_catch_type(&mut self) -> CatchType {
        let id = identifiers::parse_full_name_identifier();

        if self.current().kind == TokenKind::Pipe {
            self.next();

            let mut types = vec![id];

            while !state.is_eof() {
                let id = identifiers::parse_full_name_identifier();
                types.push(id);

                if self.current().kind != TokenKind::Pipe {
                    break;
                }

                self.next();
            }

            let span = types.span();

            return CatchType {
                id: self.state.id(),
                span,
                kind: CatchTypeKind::Union(CatchTypeKindUnion {
                    id: self.state.id(),
                    span,
                    identifiers: types,
                }),
            };
        }

        CatchType {
            id: self.state.id(),
            span: id.span(),
            kind: CatchTypeKind::Identifier(CatchTypeKindIdentifier {
                id: self.state.id(),
                span: id.span(),
                identifier: id,
            }),
        }
    }
}
