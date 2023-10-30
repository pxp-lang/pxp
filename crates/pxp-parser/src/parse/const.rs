use pxp_ast::{Statement, Constant, StatementKind, ConstantStatement};
use pxp_span::Span;
use pxp_token::TokenKind;

use crate::{state::ParserState, parse::{utils::{skip, constant_identifier, skip_semicolon}, expressions}};

use super::utils::unexpected_token;

pub fn r#const(state: &mut ParserState) -> Statement {
    let r#const = skip(state, TokenKind::Const);

    let mut constants = Vec::new();

    while !state.stream.is_eof() {
        let name = constant_identifier(state);
        let assign = skip(state, TokenKind::Assign);
        let value = expressions::create(state);
        
        constants.push(Constant {
            name,
            value
        });

        if state.stream.current().kind != TokenKind::SemiColon {
            skip(state, TokenKind::Comma);

            // const FOO = 1, ;
            if state.stream.current().kind == TokenKind::SemiColon {
                unexpected_token(state, &[TokenKind::Identifier]);
                break;
            }
        } else {
            break;
        }
    }

    let semicolon = skip_semicolon(state);

    Statement::new(
        StatementKind::Constant(ConstantStatement { constants }),
        Span::new(r#const.span.start, semicolon.span.end)
    )
}