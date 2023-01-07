use pxp_parser::{lexer::token::Span, parser::ast::Expression};

use super::Transpiler;

pub struct ShortMatchTranspiler;

impl Transpiler for ShortMatchTranspiler {
    fn transpile_expression(&mut self, expression: &mut Expression) {
        if let Expression::ShortMatch {keyword, default,arms,} = expression {
            *expression = Expression::Match {
                keyword: *keyword,
                left_parenthesis: Span::default(),
                condition: Box::new(Expression::Bool { value: true }),
                right_parenthesis: Span::default(),
                default: default.clone(),
                arms: arms.clone(),
                left_brace: Span::default(),
                right_brace: Span::default(),
            };
        }   
    }
}
