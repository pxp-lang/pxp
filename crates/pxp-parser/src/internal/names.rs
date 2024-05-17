use pxp_ast::name::Name;
use pxp_diagnostics::Severity;
use pxp_symbol::Symbol;
use pxp_token::TokenKind;

use crate::{state::State, ParserDiagnostic};

use super::identifiers::{full_type_name, is_reserved_identifier};

pub fn type_name(state: &mut State) -> Name {
    let current = state.stream.current();

    match &current.kind {
        TokenKind::Identifier | TokenKind::Enum | TokenKind::From => {
            state.stream.next();

            let symbol = current.symbol.unwrap();
            let resolved = state.join_with_namespace(symbol);

            Name::resolved(resolved, symbol, current.span)
        }
        TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
            state.diagnostic(
                ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                Severity::Error,
                current.span,
            );

            state.stream.next();

            let symbol = current.symbol.unwrap();
            let resolved = state.join_with_namespace(symbol);

            Name::resolved(resolved, symbol, current.span)
        }
        t if is_reserved_identifier(t) => {
            state.diagnostic(
                ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                Severity::Error,
                current.span,
            );

            state.stream.next();

            let symbol = current.symbol.unwrap();
            let resolved = state.join_with_namespace(symbol);

            Name::resolved(resolved, symbol, current.span)
        }
        _ => {
            state.diagnostic(
                ParserDiagnostic::ExpectedToken {
                    expected: vec![TokenKind::Identifier],
                    found: *current,
                },
                Severity::Error,
                current.span,
            );

            Name::resolved(Symbol::missing(), Symbol::missing(), current.span)
        }
    }
}

// Names inside of a `use` statement are always resolved.
pub fn use_name(state: &mut State) -> Name {
    let identifier = full_type_name(state);

    if identifier.symbol.is_missing() {
        return Name::missing(identifier.span);
    }

    Name::resolved(identifier.symbol, identifier.symbol, identifier.span)
}