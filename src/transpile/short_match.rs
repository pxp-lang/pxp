use pxp_parser::{lexer::token::Span, parser::ast::Expression};

use super::Transpiler;

pub struct ShortMatchTranspiler;

impl Transpiler for ShortMatchTranspiler {
    fn transpile_expression(&mut self, expression: &mut Expression) {
        match expression {
            Expression::ShortMatch {
                keyword,
                default,
                arms,
            } => {
                *expression = Expression::Match {
                    keyword: keyword.clone(),
                    left_parenthesis: Span::default(),
                    condition: Box::new(Expression::Bool { value: true }),
                    right_parenthesis: Span::default(),
                    default: default.clone(),
                    arms: arms.clone(),
                };
            }
            _ => return,
        }
    }
}
