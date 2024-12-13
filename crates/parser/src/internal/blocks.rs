use crate::internal::utils;
use crate::state::State;
use crate::statement;
use crate::Parser;
use pxp_ast::BlockStatement;
use pxp_ast::Statement;
use pxp_ast::StatementKind;
use pxp_span::Span;
use pxp_token::OpenTagKind;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_block_statement(&mut self) -> StatementKind {
        let (left_brace, statements, right_brace) = utils::braced(state, &|&mut self| {
            parse_multiple_statements_until(state, &TokenKind::RightBrace)
        });

        StatementKind::Block(BlockStatement {
            id: self.state.id(),
            span: Span::combine(left_brace, right_brace),
            left_brace,
            statements,
            right_brace,
        })
    }

    pub fn parse_multiple_statements_until(&mut self, until: &TokenKind) -> Vec<Statement> {
        let mut statements = Vec::new();

        let mut current = self.current();
        while &current.kind != until {
            if let TokenKind::OpenTag(OpenTagKind::Full) = current.kind {
                self.next();

                current = self.current();
                continue;
            }

            statements.push(statement());
            current = self.current();
        }

        statements
    }

    pub fn parse_multiple_statements_until_any(&mut self, until: &[TokenKind]) -> Vec<Statement> {
        let mut statements = Vec::new();

        let mut current = self.current();
        while !until.contains(&current.kind) {
            if let TokenKind::OpenTag(OpenTagKind::Full) = current.kind {
                self.next();

                current = self.current();
                continue;
            }

            statements.push(statement());
            current = self.current();
        }

        statements
    }
}
