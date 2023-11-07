use crate::lexer::token::TokenKind;
use crate::parser::ast::GroupUseStatement;
use crate::parser::ast::Statement;
use crate::parser::ast::Use;
use crate::parser::ast::UseKind;
use crate::parser::ast::UseStatement;
use crate::parser::error;
use crate::parser::error::ParseResult;
use crate::parser::internal::identifiers;
use crate::parser::internal::utils;
use crate::parser::state::State;

pub fn use_statement(state: &mut State) -> ParseResult<Statement> {
    state.stream.next();

    let kind = match state.stream.current().kind {
        TokenKind::Function => {
            state.stream.next();
            UseKind::Function
        }
        TokenKind::Const => {
            state.stream.next();
            UseKind::Const
        }
        _ => UseKind::Normal,
    };

    if state.stream.peek().kind == TokenKind::LeftBrace {
        let prefix = identifiers::full_name(state)?;
        state.stream.next();
        let mut uses = Vec::new();
        while state.stream.current().kind != TokenKind::RightBrace {
            let use_kind = match state.stream.current().kind {
                TokenKind::Function => {
                    if kind != UseKind::Normal {
                        return Err(error::unexpected_token(
                            vec!["an identifier".to_string()],
                            state.stream.current(),
                        ));
                    }

                    state.stream.next();
                    Some(UseKind::Function)
                }
                TokenKind::Const => {
                    if kind != UseKind::Normal {
                        return Err(error::unexpected_token(
                            vec!["an identifier".to_string()],
                            state.stream.current(),
                        ));
                    }

                    state.stream.next();
                    Some(UseKind::Const)
                }
                _ => None,
            };

            let name = identifiers::full_type_name(state)?;
            let mut alias = None;
            if state.stream.current().kind == TokenKind::As {
                state.stream.next();
                alias = Some(identifiers::type_identifier(state)?);
            }

            uses.push(Use {
                name,
                kind: use_kind,
                alias,
            });

            if state.stream.current().kind == TokenKind::Comma {
                state.stream.next();
                continue;
            }
        }

        utils::skip_right_brace(state)?;
        utils::skip_semicolon(state)?;

        Ok(Statement::GroupUse(GroupUseStatement {
            prefix,
            kind,
            uses,
        }))
    } else {
        let mut uses = Vec::new();
        while !state.stream.is_eof() {
            let name = identifiers::full_type_name(state)?;
            let mut alias = None;
            if state.stream.current().kind == TokenKind::As {
                state.stream.next();
                alias = Some(identifiers::type_identifier(state)?);
            }

            uses.push(Use {
                name,
                kind: None,
                alias,
            });

            if state.stream.current().kind == TokenKind::Comma {
                state.stream.next();
                continue;
            }

            utils::skip_semicolon(state)?;
            break;
        }

        Ok(Statement::Use(UseStatement { uses, kind }))
    }
}
