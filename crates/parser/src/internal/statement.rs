use pxp_ast::Statement;
use pxp_token::TokenKind;

use crate::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_top_level_statement(&mut self) -> Statement {
        match self.current_kind() {
            TokenKind::Namespace => self.parse_namespace(),
            TokenKind::Const => self.parse_const(),
            TokenKind::Use => self.parse_use(),
            TokenKind::HaltCompiler => self.parse_halt_compiler(),
            _ => self.parse_statement(),
        }
    }

    pub(crate) fn parse_statement(&mut self) -> Statement {
        match self.current_kind() {
            TokenKind::OpenTag(_) => self.parse_open_tag(),
            _ => todo!(),
        }
    }
}
