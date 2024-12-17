use name::NameQualification;
use pxp_ast::*;
use pxp_bytestring::ByteString;
use pxp_diagnostics::Severity;
use pxp_token::TokenKind;

use crate::{Parser, ParserDiagnostic};

impl<'a> Parser<'a> {
    pub fn parse_full_name(&mut self, kind: UseKind) -> Name {
        match self.current_kind() {
            TokenKind::FullyQualifiedIdentifier => self.next_but_first(|parser| {
                Name::resolved(
                    parser.id(),
                    parser
                        .state
                        .strip_leading_namespace_qualifier(&parser.current_symbol_as_bytestring()),
                    parser.current_symbol_as_bytestring(),
                    parser.current_span(),
                )
            }),
            TokenKind::Identifier | TokenKind::QualifiedIdentifier => {
                self.next_but_first(|parser| {
                    let id = parser.id();

                    parser.maybe_resolve_identifier(id, &parser.current(), kind)
                })
            }
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: self.current().to_owned(),
                    },
                    Severity::Error,
                    self.current_span(),
                );

                Name::missing(self.id(), self.current_span())
            }
        }
    }

    pub fn parse_type_name_maybe_soft_reserved(&mut self) -> Name {
        if self.is_soft_reserved_identifier(self.current_kind()) {
            let symbol = self.current_symbol_as_bytestring();
            let resolved = self.state.join_with_namespace(&symbol);
            let span = self.current_span();

            self.next();

            Name::resolved(self.id(), resolved, symbol, span)
        } else {
            self.parse_type_name()
        }
    }

    pub fn parse_name_maybe_soft_reserved(&mut self, kind: UseKind) -> Name {
        if self.is_soft_reserved_identifier(self.current_kind()) {
            self.next_but_first(|parser| {
                let id = parser.id();

                parser.maybe_resolve_identifier(id, &parser.current(), kind)
            })
        } else {
            self.parse_full_name(kind)
        }
    }

    pub fn parse_type_name(&mut self) -> Name {
        match self.current_kind() {
            TokenKind::Identifier | TokenKind::Enum | TokenKind::From => {
                self.next_but_first(|parser| {
                    let symbol = parser.current_symbol_as_bytestring();
                    let resolved = parser.state.join_with_namespace(&symbol);

                    Name::resolved(parser.id(), resolved, symbol, parser.current_span())
                })
            }
            TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
                self.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                    Severity::Error,
                    self.current_span(),
                );

                self.next_but_first(|parser| {
                    let symbol = parser.current_symbol_as_bytestring();
                    let resolved = parser.state.join_with_namespace(&symbol);

                    Name::resolved(parser.id(), resolved, symbol, parser.current_span())
                })
            }
            t if self.is_reserved_identifier(t) => {
                self.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                    Severity::Error,
                    self.current_span(),
                );

                self.next_but_first(|parser| {
                    let symbol = parser.current_symbol_as_bytestring();
                    let resolved = parser.state.join_with_namespace(&symbol);

                    Name::resolved(parser.id(), resolved, symbol, parser.current_span())
                })
            }
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: self.current().to_owned(),
                    },
                    Severity::Error,
                    self.current_span(),
                );

                Name::resolved(
                    self.id(),
                    ByteString::empty(),
                    ByteString::empty(),
                    self.current_span(),
                )
            }
        }
    }

    // Names inside of a `use` statement are always resolved.
    pub fn parse_use_name(&mut self) -> Name {
        let identifier = self.parse_full_type_name_identifier();

        if identifier.symbol.is_empty() {
            return Name::missing(self.id(), identifier.span);
        }

        Name::resolved(
            self.id(),
            identifier.symbol.clone(),
            identifier.symbol,
            identifier.span,
        )
    }

    pub fn parse_full_name_including_self(&mut self) -> Name {
        match self.current_kind() {
            TokenKind::FullyQualifiedIdentifier => self.next_but_first(|parser| {
                let symbol = parser.current_symbol_as_bytestring();
                let resolved = parser.state.strip_leading_namespace_qualifier(&symbol);

                Name::resolved(parser.id(), resolved, symbol, parser.current_span())
            }),
            TokenKind::Identifier
            | TokenKind::QualifiedIdentifier
            | TokenKind::Enum
            | TokenKind::From => self.next_but_first(|parser| {
                let id = parser.id();

                parser.maybe_resolve_identifier(id, &parser.current(), UseKind::Normal)
            }),
            TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
                self.next_but_first(|parser| {
                    let symbol = parser.current_symbol_as_bytestring();

                    Name::special(
                        parser.id(),
                        SpecialNameKind::from(parser.current()),
                        symbol,
                        parser.current_span(),
                    )
                })
            }
            t if self.is_reserved_identifier(t) => {
                self.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                    Severity::Error,
                    self.current_span(),
                );

                self.next_but_first(|parser| {
                    let symbol = parser.current_symbol_as_bytestring();

                    Name::unresolved(
                        parser.id(),
                        symbol,
                        NameQualification::Unqualified,
                        parser.current_span(),
                    )
                })
            }
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: self.current().to_owned(),
                    },
                    Severity::Error,
                    self.current_span(),
                );

                Name::missing(self.id(), self.current_span())
            }
        }
    }

    pub fn parse_constant_identifier(&mut self) -> Name {
        match self.current_kind() {
            TokenKind::Identifier
            | TokenKind::Enum
            | TokenKind::From
            | TokenKind::Self_
            | TokenKind::Parent => self.next_but_first(|parser| {
                let symbol = parser.current_symbol_as_bytestring();
                let resolved = parser.state.join_with_namespace(&symbol);

                Name::resolved(parser.id(), resolved, symbol, parser.current_span())
            }),
            t if self.is_reserved_identifier(t) => {
                self.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsConstantName,
                    Severity::Error,
                    self.current_span(),
                );

                self.next_but_first(|parser| {
                    let symbol = parser.current_symbol_as_bytestring();
                    let resolved = parser.state.join_with_namespace(&symbol);

                    Name::resolved(parser.id(), resolved, symbol, parser.current_span())
                })
            }
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: self.current().to_owned(),
                    },
                    Severity::Error,
                    self.current_span(),
                );

                Name::missing(self.id(), self.current_span())
            }
        }
    }
}
