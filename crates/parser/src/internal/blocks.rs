use crate::Parser;
use pxp_ast::BlockStatement;
use pxp_ast::Statement;
use pxp_ast::StatementKind;
use pxp_span::Span;
use pxp_token::OpenTagKind;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_block_statement(&mut self) -> StatementKind {
        let (left_brace, statements, right_brace) =
            self.braced(|parser| parser.parse_multiple_statements_until(TokenKind::RightBrace));

        StatementKind::Block(BlockStatement {
            id: self.state.id(),
            span: Span::combine(left_brace, right_brace),
            left_brace,
            statements,
            right_brace,
        })
    }

    pub fn parse_multiple_statements_until(&mut self, until: TokenKind) -> Vec<Statement> {
        let mut statements = Vec::new();

        while self.current_kind() != until {
            if let TokenKind::OpenTag(OpenTagKind::Full) = self.current_kind() {
                self.next();

                continue;
            }

            statements.push(self.parse_statement());
        }

        statements
    }

    pub fn parse_multiple_statements_until_any(&mut self, until: &[TokenKind]) -> Vec<Statement> {
        let mut statements = Vec::new();

        while !until.contains(&self.current_kind()) {
            if let TokenKind::OpenTag(OpenTagKind::Full) = self.current_kind() {
                self.next();

                continue;
            }

            statements.push(self.parse_statement());
        }

        statements
    }
}
