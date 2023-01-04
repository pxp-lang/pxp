use pxp_parser::parser::ast::{Expression, Statement};

pub mod short_match;

pub trait Transpiler {
    fn transpile_statement(&mut self, statement: &mut Statement) {}
    fn transpile_expression(&mut self, expression: &mut Expression) {}
}
