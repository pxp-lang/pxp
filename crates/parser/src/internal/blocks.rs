use crate::Parser;
use pxp_ast::BlockStatement;
use pxp_ast::Statement;
use pxp_ast::StatementKind;
use pxp_span::Span;
use pxp_token::OpenTagKind;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn block_statement(&mut self) -> StatementKind {
        let (left_brace, statements, right_brace) = self.braced(&|parser: &mut Parser| {
            parser.multiple_statements_until(&TokenKind::RightBrace)
        });

        StatementKind::Block(BlockStatement {
            id: self.state.id(),
            span: Span::combine(left_brace, right_brace),
            left_brace,
            statements,
            right_brace,
        })
    }

    pub fn multiple_statements_until(&mut self, until: &TokenKind) -> Vec<Statement> {
        let mut statements = Vec::new();

        let mut current = self.state.current();
        while &current.kind != until {
            if let TokenKind::OpenTag(OpenTagKind::Full) = current.kind {
                self.state.next();

                current = self.state.current();
                continue;
            }

            statements.push(self.statement());
            current = self.state.current();
        }

        statements
    }

    pub fn multiple_statements_until_any(&mut self, until: &[TokenKind]) -> Vec<Statement> {
        let mut statements = Vec::new();

        let mut current = self.state.current();
        while !until.contains(&current.kind) {
            if let TokenKind::OpenTag(OpenTagKind::Full) = current.kind {
                self.state.next();

                current = self.state.current();
                continue;
            }

            statements.push(self.statement());
            current = self.state.current();
        }

        statements
    }
}