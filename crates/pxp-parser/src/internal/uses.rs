use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use crate::ParserDiagnostic;
use pxp_ast::*;
use pxp_ast::GroupUseStatement;
use pxp_ast::StatementKind;
use pxp_ast::Use;
use pxp_ast::UseKind;
use pxp_ast::UseStatement;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;

use super::names;

pub fn use_statement(state: &mut State) -> StatementKind {
    let r#use = state.stream.current().span;

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
                 id: state.id(), 
                name: Name::resolved(state.id(), state.symbol_table.coagulate(&[prefix.symbol, name.symbol], Some(b"\\")), name.symbol, name.span),
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
        let semicolon = utils::skip_semicolon(state);

        StatementKind::GroupUse(GroupUseStatement {  id: state.id(), span: Span::combine(prefix.span, semicolon), prefix, kind, uses })
    } else {
        let mut uses = Vec::new();
        while !state.stream.is_eof() {
            let name = names::use_name(state);
            let mut alias = None;
            if state.stream.current().kind == TokenKind::As {
                state.stream.next();
                alias = Some(identifiers::type_identifier(state));
            }

            let alias_symbol = alias.as_ref().map(|a| a.symbol);

            uses.push(Use {
                 id: state.id(), 
                name,
                kind: None,
                alias,
            });

            state.add_import(&kind, name.symbol(), alias_symbol);

            if state.stream.current().kind == TokenKind::Comma {
                state.stream.next();
                continue;
            }

            utils::skip_semicolon(state);
            break;
        }

        let span = Span::combine(r#use, state.stream.previous().span);

        StatementKind::Use(UseStatement {  id: state.id(), span, uses, kind })
    }
}
