use name::NameQualification;
use pxp_ast::*;
use pxp_bytestring::ByteString;
use pxp_diagnostics::Severity;
use pxp_token::TokenKind;

use crate::{state::State, Parser, ParserDiagnostic};

use super::identifiers::{self, is_reserved_identifier, is_soft_reserved_identifier};

impl<'a> Parser<'a> {
    pub fn parse_full_name(&mut self, kind: UseKind) -> Name {
        let current = state.current();

        match &current.kind {
            TokenKind::FullyQualifiedIdentifier => {
                state.next();

                let symbol = current.symbol.as_ref().unwrap();
                let resolved = state.strip_leading_namespace_qualifier(symbol);

                Name::resolved(state.id(), resolved, symbol.clone(), current.span)
            }
            TokenKind::Identifier | TokenKind::QualifiedIdentifier => {
                state.next();

                state.maybe_resolve_identifier(current, kind)
            }
            _ => {
                state.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: current.clone(),
                    },
                    Severity::Error,
                    current.span,
                );

                Name::missing(state.id(), current.span)
            }
        }
    }

    pub fn parse_type_name_maybe_soft_reserved(&mut self) -> Name {
        let current = state.current();

        if is_soft_reserved_identifier(&current.kind) {
            let symbol = current.symbol.as_ref().unwrap();
            let resolved = state.join_with_namespace(symbol);

            state.next();

            Name::resolved(state.id(), resolved, symbol.clone(), current.span)
        } else {
            parse_type_name(state)
        }
    }

    pub fn parse_name_maybe_soft_reserved(&mut self, kind: UseKind) -> Name {
        let current = state.current();

        if is_soft_reserved_identifier(&current.kind) {
            state.next();

            state.maybe_resolve_identifier(current, kind)
        } else {
            parse_full_name(state, kind)
        }
    }

    pub fn parse_type_name(&mut self) -> Name {
        let current = state.current();

        match &current.kind {
            TokenKind::Identifier | TokenKind::Enum | TokenKind::From => {
                state.next();

                let symbol = current.symbol.as_ref().unwrap();
                let resolved = state.join_with_namespace(symbol);

                Name::resolved(state.id(), resolved, symbol.clone(), current.span)
            }
            TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
                state.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                    Severity::Error,
                    current.span,
                );

                state.next();

                let symbol = current.symbol.as_ref().unwrap();
                let resolved = state.join_with_namespace(symbol);

                Name::resolved(state.id(), resolved, symbol.clone(), current.span)
            }
            t if is_reserved_identifier(t) => {
                state.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                    Severity::Error,
                    current.span,
                );

                state.next();

                let symbol = current.symbol.as_ref().unwrap();
                let resolved = state.join_with_namespace(symbol);

                Name::resolved(state.id(), resolved, symbol.clone(), current.span)
            }
            _ => {
                state.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: current.clone(),
                    },
                    Severity::Error,
                    current.span,
                );

                Name::resolved(
                    state.id(),
                    ByteString::empty(),
                    ByteString::empty(),
                    current.span,
                )
            }
        }
    }

    // Names inside of a `use` statement are always resolved.
    pub fn parse_use_name(&mut self) -> Name {
        let identifier = identifiers::parse_full_type_name_identifier(state);

        if identifier.symbol.is_empty() {
            return Name::missing(state.id(), identifier.span);
        }

        Name::resolved(
            state.id(),
            identifier.symbol.clone(),
            identifier.symbol,
            identifier.span,
        )
    }

    pub fn parse_full_name_including_self(&mut self) -> Name {
        let current = state.current();
        match &current.kind {
            TokenKind::FullyQualifiedIdentifier => {
                state.next();

                let symbol = current.symbol.as_ref().unwrap();
                let resolved = state.strip_leading_namespace_qualifier(symbol);

                Name::resolved(state.id(), resolved, symbol.clone(), current.span)
            }
            TokenKind::Identifier
            | TokenKind::QualifiedIdentifier
            | TokenKind::Enum
            | TokenKind::From => {
                state.next();

                state.maybe_resolve_identifier(current, UseKind::Normal)
            }
            TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
                state.next();

                let symbol = current.symbol.as_ref().unwrap();

                Name::special(
                    state.id(),
                    SpecialNameKind::from(current.clone()),
                    symbol.clone(),
                    current.span,
                )
            }
            t if is_reserved_identifier(t) => {
                state.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                    Severity::Error,
                    current.span,
                );

                state.next();

                let symbol = current.symbol.as_ref().unwrap();

                Name::unresolved(
                    state.id(),
                    symbol.clone(),
                    NameQualification::Unqualified,
                    current.span,
                )
            }
            _ => {
                state.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: current.clone(),
                    },
                    Severity::Error,
                    current.span,
                );

                Name::missing(state.id(), current.span)
            }
        }
    }

    pub fn parse_constant_identifier(&mut self) -> Name {
        let current = state.current();
        match &current.kind {
            TokenKind::Identifier
            | TokenKind::Enum
            | TokenKind::From
            | TokenKind::Self_
            | TokenKind::Parent => {
                state.next();

                let symbol = current.symbol.as_ref().unwrap();
                let resolved = state.join_with_namespace(symbol);

                Name::resolved(state.id(), resolved, symbol.clone(), current.span)
            }
            t if is_reserved_identifier(t) => {
                state.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsConstantName,
                    Severity::Error,
                    current.span,
                );

                state.next();

                let symbol = current.symbol.as_ref().unwrap();
                let resolved = state.join_with_namespace(symbol);

                Name::resolved(state.id(), resolved, symbol.clone(), current.span)
            }
            _ => {
                state.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: current.clone(),
                    },
                    Severity::Error,
                    current.span,
                );

                Name::missing(state.id(), current.span)
            }
        }
    }
}
