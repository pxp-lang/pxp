use crate::Parser;
use crate::ParserDiagnostic;
use pxp_ast::StatementKind;
use pxp_ast::*;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_try_block(&mut self) -> StatementKind {
        let start = self.current_span();

        self.next();
        self.skip_left_brace();

        let body = self.parse_multiple_statements_until(TokenKind::RightBrace);

        let last_right_brace = self.skip_right_brace();

        let mut catches = Vec::new();
        loop {
            if self.current_kind() != TokenKind::Catch {
                break;
            }

            let catch_start = self.current_span();

            self.next();
            self.skip_left_parenthesis();

            let types = self.parse_catch_type();
            let var = if self.current_kind() == TokenKind::RightParen {
                None
            } else {
                Some(self.parse_simple_variable())
            };

            self.skip_right_parenthesis();
            self.skip_left_brace();

            let catch_body = self.parse_multiple_statements_until(TokenKind::RightBrace);

            self.skip_right_brace();

            let catch_end = self.current_span();

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
        if self.current_kind() == TokenKind::Finally {
            let finally_start = self.current_span();
            self.next();
            self.skip_left_brace();

            let finally_body =
                self.parse_multiple_statements_until(TokenKind::RightBrace);

            self.skip_right_brace();
            let finally_end = self.current_span();

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

        let span = if finally.is_some() {
            Span::combine(start, finally.span())
        } else if !catches.is_empty() {
            Span::combine(start, catches.span())
        } else {
            Span::combine(start, last_right_brace)
        };

        StatementKind::Try(TryStatement {
            id: self.state.id(),
            span,
            start,
            end: last_right_brace,
            body,
            catches,
            finally,
        })
    }

    #[inline(always)]
    fn parse_catch_type(&mut self) -> CatchType {
        let id = self.parse_full_name_identifier();

        if self.current_kind() == TokenKind::Pipe {
            self.next();

            let mut types = vec![id];

            while !self.is_eof() {
                let id = self.parse_full_name_identifier();
                types.push(id);

                if self.current_kind() != TokenKind::Pipe {
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
