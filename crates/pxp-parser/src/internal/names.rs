use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_symbol::Symbol;
use pxp_syntax::name::NameQualification;
use pxp_token::TokenKind;

use crate::{state::State, ParserDiagnostic};

use super::identifiers::{self, is_reserved_identifier, is_soft_reserved_identifier};

pub fn full_name(state: &mut State, kind: UseKind) -> Name {
    let current = state.stream.current();

    match &current.kind {
        TokenKind::FullyQualifiedIdentifier => {
            state.stream.next();

            let symbol = current.symbol.unwrap();
            let resolved = state.strip_leading_namespace_qualifier(symbol);

            Name::resolved(state.id(), resolved, symbol, current.span)
        }
        TokenKind::Identifier | TokenKind::QualifiedIdentifier => {
            state.stream.next();

            state.maybe_resolve_identifier(*current, kind)
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

            Name::missing(state.id(), current.span)
        }
    }
}

pub fn type_name_maybe_soft_reserved(state: &mut State) -> Name {
    let current = state.stream.current();

    if is_soft_reserved_identifier(&current.kind) {
        let symbol = current.symbol.unwrap();
        let resolved = state.join_with_namespace(symbol);

        state.stream.next();

        Name::resolved(state.id(), resolved, symbol, current.span)
    } else {
        type_name(state)
    }
}

pub fn name_maybe_soft_reserved(state: &mut State, kind: UseKind) -> Name {
    let current = state.stream.current();

    if is_soft_reserved_identifier(&current.kind) {
        state.stream.next();

        state.maybe_resolve_identifier(*current, kind)
    } else {
        full_name(state, kind)
    }
}

pub fn type_name(state: &mut State) -> Name {
    let current = state.stream.current();

    match &current.kind {
        TokenKind::Identifier | TokenKind::Enum | TokenKind::From => {
            state.stream.next();

            let symbol = current.symbol.unwrap();
            let resolved = state.join_with_namespace(symbol);

            Name::resolved(state.id(), resolved, symbol, current.span)
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

            Name::resolved(state.id(), resolved, symbol, current.span)
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

            Name::resolved(state.id(), resolved, symbol, current.span)
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

            Name::resolved(
                state.id(),
                Symbol::missing(),
                Symbol::missing(),
                current.span,
            )
        }
    }
}

// Names inside of a `use` statement are always resolved.
pub fn use_name(state: &mut State) -> Name {
    let identifier = identifiers::full_type_name(state);

    if identifier.symbol.is_missing() {
        return Name::missing(state.id(), identifier.span);
    }

    Name::resolved(
        state.id(),
        identifier.symbol,
        identifier.symbol,
        identifier.span,
    )
}

pub fn full_name_including_self(state: &mut State) -> Name {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::FullyQualifiedIdentifier => {
            state.stream.next();

            let symbol = current.symbol.unwrap();
            let resolved = state.strip_leading_namespace_qualifier(symbol);

            Name::resolved(state.id(), resolved, symbol, current.span)
        }
        TokenKind::Identifier
        | TokenKind::QualifiedIdentifier
        | TokenKind::Enum
        | TokenKind::From => {
            state.stream.next();

            state.maybe_resolve_identifier(*current, UseKind::Normal)
        }
        TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
            state.stream.next();

            let symbol = current.symbol.unwrap();

            Name::special(
                state.id(),
                SpecialNameKind::from(*current),
                symbol,
                current.span,
            )
        }
        t if is_reserved_identifier(t) => {
            state.diagnostic(
                ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                Severity::Error,
                current.span,
            );

            state.stream.next();

            let symbol = current.symbol.unwrap();

            Name::unresolved(
                state.id(),
                symbol,
                NameQualification::Unqualified,
                current.span,
            )
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

            Name::missing(state.id(), current.span)
        }
    }
}

pub fn constant_identifier(state: &mut State) -> Name {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier
        | TokenKind::Enum
        | TokenKind::From
        | TokenKind::Self_
        | TokenKind::Parent => {
            state.stream.next();

            let symbol = current.symbol.unwrap();
            let resolved = state.join_with_namespace(symbol);

            Name::resolved(state.id(), resolved, symbol, current.span)
        }
        t if is_reserved_identifier(t) => {
            state.diagnostic(
                ParserDiagnostic::CannotUseReservedKeywordAsConstantName,
                Severity::Error,
                current.span,
            );

            state.stream.next();

            let symbol = current.symbol.unwrap();
            let resolved = state.join_with_namespace(symbol);

            Name::resolved(state.id(), resolved, symbol, current.span)
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

            Name::missing(state.id(), current.span)
        }
    }
}
