use pxp_ast::{Expression, ExpressionKind, InterpolatedStringExpression, StringPart, StringPartKind};
use pxp_token::TokenKind;

use crate::state::ParserState;

use super::{variables, utils::{self, unexpected_token}, create};

#[inline(always)]
pub fn interpolated(state: &mut ParserState) -> Expression {
    let start = state.stream.current().span.start;
    let mut parts = Vec::new();

    while state.stream.current().kind != TokenKind::DoubleQuote {
        if let Some(part) = part(state) {
            parts.push(part);
        }
    }

    let end = state.stream.current().span.end;
    state.stream.next();

    Expression::new(
        ExpressionKind::InterpolatedString(InterpolatedStringExpression { parts }),
        (start, end).into()
    )
}

fn part(state: &mut ParserState) -> Option<StringPart> {
    match &state.stream.current().kind {
        TokenKind::InterpolatedStringPart => {
            let s = state.stream.current().clone();
            let part = if s.literal.len() > 0 {
                Some(StringPart::literal(s))
            } else {
                None
            };

            state.stream.next();
            part
        }
        TokenKind::DollarLeftBrace => {
            let variable = variables::dynamic_variable(state);

            Some(StringPart {
                kind: StringPartKind::Variable(Box::new(variable)),
            })
        }
        TokenKind::LeftBrace => {
            // "{$expr}"
            state.stream.next();
            let e = create(state);
            utils::skip_right_brace(state);
            Some(StringPart { kind: StringPartKind::Expression(Box::new(e)) })
        }
        TokenKind::Variable => {
            // "$expr", "$expr[0]", "$expr[name]", "$expr->a"
            // FIXME: Not all expressions are allowed here, but we want to tolerate them.
            //        We should really be entering some sort of new "scope" and emitting an error
            //        if we encounter an invalid expression.
            let e = create(state);

            Some(StringPart {
                kind: StringPartKind::Expression(Box::new(e))
            })
        }
        _ => {
            let span = state.stream.current().span;

            unexpected_token(state, &[TokenKind::Variable, TokenKind::DoubleQuote, TokenKind::DollarLeftBrace]);
            
            Some(StringPart {
                kind: StringPartKind::Expression(Box::new(Expression::missing(span)))
            })
        }
    }
}