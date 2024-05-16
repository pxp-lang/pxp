use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use crate::ParserDiagnostic;
use pxp_ast::GroupUseStatement;
use pxp_ast::StatementKind;
use pxp_ast::Use;
use pxp_ast::UseKind;
use pxp_ast::UseStatement;
use pxp_diagnostics::Severity;
use pxp_token::TokenKind;

pub fn use_statement(state: &mut State) -> StatementKind {
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
        let prefix = identifiers::full_name(state);
        let prefix_symbol = prefix.symbol;

        state.stream.next();
        
        let mut uses = Vec::new();
        while state.stream.current().kind != TokenKind::RightBrace {
            let use_kind = match state.stream.current().kind {
                TokenKind::Function => {
                    if kind != UseKind::Normal {
                        state.diagnostic(ParserDiagnostic::MixedImportTypes, Severity::Error, state.stream.current().span);
                    }

                    state.stream.next();
                    Some(UseKind::Function)
                }
                TokenKind::Const => {
                    if kind != UseKind::Normal {
                        state.diagnostic(ParserDiagnostic::MixedImportTypes, Severity::Error, state.stream.current().span);
                    }

                    state.stream.next();
                    Some(UseKind::Const)
                }
                _ => None,
            };

            let name = identifiers::full_type_name(state);
            let mut alias = None;
            if state.stream.current().kind == TokenKind::As {
                state.stream.next();
                alias = Some(identifiers::type_identifier(state));
            }

            let symbol = name.symbol;
            let alias_symbol = alias.as_ref().map(|a| a.symbol);
            let import_kind = use_kind.unwrap_or(kind);

            uses.push(Use {
                name,
                kind: use_kind,
                alias,
            });

            state.add_prefixed_import(&import_kind, prefix_symbol, symbol, alias_symbol);

            if state.stream.current().kind == TokenKind::Comma {
                state.stream.next();
                continue;
            }
        }

        utils::skip_right_brace(state);
        utils::skip_semicolon(state);

        StatementKind::GroupUse(GroupUseStatement { prefix, kind, uses })
    } else {
        let mut uses = Vec::new();
        while !state.stream.is_eof() {
            let name = identifiers::full_type_name(state);
            let mut alias = None;
            if state.stream.current().kind == TokenKind::As {
                state.stream.next();
                alias = Some(identifiers::type_identifier(state));
            }

            let symbol = name.symbol;
            let alias_symbol = alias.as_ref().map(|a| a.symbol);

            uses.push(Use {
                name,
                kind: None,
                alias,
            });

            state.add_import(&kind, symbol, alias_symbol);

            if state.stream.current().kind == TokenKind::Comma {
                state.stream.next();
                continue;
            }

            utils::skip_semicolon(state);
            break;
        }

        StatementKind::Use(UseStatement { uses, kind })
    }
}
