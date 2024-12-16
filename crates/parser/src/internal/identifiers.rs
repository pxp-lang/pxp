use crate::{Parser, ParserDiagnostic};
use pxp_ast::*;
use pxp_bytestring::ByteString;
use pxp_diagnostics::Severity;

use pxp_span::Span;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    /// Expect an unqualified identifier such as Foo or Bar for a class, interface, trait, or an enum name.
    pub fn parse_type_identifier(&mut self) -> SimpleIdentifier {
        match self.current_kind() {
            TokenKind::Identifier | TokenKind::Enum | TokenKind::From => {
                self.next_but_first(|parser| {
                    SimpleIdentifier::new(
                        parser.state.id(),
                        parser.current_symbol_as_bytestring(),
                        parser.current_span(),
                    )
                })
            }
            TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
                self.next_but_first(|parser| {
                    parser.diagnostic(
                        ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                        Severity::Error,
                        parser.current_span(),
                    );

                    SimpleIdentifier::new(
                        parser.state.id(),
                        parser.current_symbol_as_bytestring(),
                        parser.current_span(),
                    )
                })
            }
            t if self.is_reserved_identifier(t) => self.next_but_first(|parser| {
                parser.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                    Severity::Error,
                    parser.current_span(),
                );

                SimpleIdentifier::new(
                    parser.state.id(),
                    parser.current_symbol_as_bytestring(),
                    parser.current_span(),
                )
            }),
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: self.current().to_owned(),
                    },
                    Severity::Error,
                    self.current_span(),
                );

                SimpleIdentifier::new(self.state.id(), ByteString::empty(), self.current_span())
            }
        }
    }

    /// Expect an unqualified identifier such as foo or bar for a goto label name.
    pub fn parse_label_identifier(&mut self) -> SimpleIdentifier {
        match self.current_kind() {
            TokenKind::Identifier | TokenKind::Enum | TokenKind::From => {
                self.next_but_first(|parser| {
                    SimpleIdentifier::new(
                        parser.state.id(),
                        parser.current_symbol_as_bytestring(),
                        parser.current_span(),
                    )
                })
            }
            TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
                self.next_but_first(|parser| {
                    parser.diagnostic(
                        ParserDiagnostic::CannotUseReservedKeywordAsLabel,
                        Severity::Error,
                        parser.current_span(),
                    );

                    SimpleIdentifier::new(
                        parser.state.id(),
                        parser.current_symbol_as_bytestring(),
                        parser.current_span(),
                    )
                })
            }
            t if self.is_reserved_identifier(t) => self.next_but_first(|parser| {
                parser.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsLabel,
                    Severity::Error,
                    parser.current_span(),
                );

                SimpleIdentifier::new(
                    parser.state.id(),
                    parser.current_symbol_as_bytestring(),
                    parser.current_span(),
                )
            }),
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: self.current().to_owned(),
                    },
                    Severity::Error,
                    self.current_span(),
                );

                SimpleIdentifier::new(self.state.id(), ByteString::empty(), self.current_span())
            }
        }
    }

    /// Expect an unqualified identifier such as Foo or Bar.
    pub fn parse_identifier(&mut self) -> SimpleIdentifier {
        if self.current_kind() == TokenKind::Identifier {
            self.next_but_first(|parser| {
                SimpleIdentifier::new(
                    parser.state.id(),
                    parser.current_symbol_as_bytestring(),
                    parser.current_span(),
                )
            })
        } else {
            self.diagnostic(
                ParserDiagnostic::UnexpectedToken {
                    token: self.current().to_owned(),
                },
                Severity::Error,
                self.current_span(),
            );

            self.next();

            // Because identifiers cannot contain spaces, we can assume that the next identifier starts
            // one byte after the previous token ends.
            SimpleIdentifier::new(self.state.id(), ByteString::empty(), Span::missing())
        }
    }

    /// Expect an unqualified or qualified identifier such as Foo, Bar or Foo\Bar.
    pub fn parse_name_identifier(&mut self) -> SimpleIdentifier {
        match self.current_kind() {
            TokenKind::Identifier | TokenKind::QualifiedIdentifier => {
                self.next_but_first(|parser| {
                    SimpleIdentifier::new(
                        parser.state.id(),
                        parser.current_symbol_as_bytestring(),
                        parser.current_span(),
                    )
                })
            }
            _ => self.next_but_first(|parser| {
                parser.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier, TokenKind::QualifiedIdentifier],
                        found: parser.current().to_owned(),
                    },
                    Severity::Error,
                    parser.current_span(),
                );

                SimpleIdentifier::new(parser.state.id(), ByteString::empty(), Span::missing())
            }),
        }
    }

    /// Expect an optional unqualified or qualified identifier such as Foo, Bar or Foo\Bar.
    pub fn parse_optional_name_identifier(&mut self) -> Option<SimpleIdentifier> {
        match self.current_kind() {
            TokenKind::Identifier | TokenKind::QualifiedIdentifier => {
                self.next_but_first(|parser| {
                    Some(SimpleIdentifier::new(
                        parser.state.id(),
                        parser.current_symbol_as_bytestring(),
                        parser.current_span(),
                    ))
                })
            }
            t if self.is_reserved_identifier(t) => self.next_but_first(|parser| {
                Some(SimpleIdentifier::new(
                    parser.state.id(),
                    parser.current_symbol_as_bytestring(),
                    parser.current_span(),
                ))
            }),
            _ => None,
        }
    }

    /// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar.
    pub fn parse_full_name_identifier(&mut self) -> SimpleIdentifier {
        match self.current_kind() {
            TokenKind::Identifier
            | TokenKind::QualifiedIdentifier
            | TokenKind::FullyQualifiedIdentifier => self.next_but_first(|parser| {
                SimpleIdentifier::new(
                    parser.state.id(),
                    parser.current_symbol_as_bytestring(),
                    parser.current_span(),
                )
            }),
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: self.current().to_owned(),
                    },
                    Severity::Error,
                    self.current_span(),
                );

                SimpleIdentifier::new(self.state.id(), ByteString::empty(), self.current_span())
            }
        }
    }

    /// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar.
    pub fn parse_full_type_name_identifier(&mut self) -> SimpleIdentifier {
        match self.current_kind() {
            TokenKind::Identifier
            | TokenKind::QualifiedIdentifier
            | TokenKind::FullyQualifiedIdentifier
            | TokenKind::Enum
            | TokenKind::From => self.next_but_first(|parser| {
                SimpleIdentifier::new(
                    parser.state.id(),
                    parser.current_symbol_as_bytestring(),
                    parser.current_span(),
                )
            }),
            TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
                self.next_but_first(|parser| {
                    parser.diagnostic(
                        ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                        Severity::Error,
                        parser.current_span(),
                    );

                    SimpleIdentifier::new(
                        parser.state.id(),
                        parser.current_symbol_as_bytestring(),
                        parser.current_span(),
                    )
                })
            }
            t if self.is_reserved_identifier(t) => self.next_but_first(|parser| {
                parser.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                    Severity::Error,
                    parser.current_span(),
                );

                SimpleIdentifier::new(
                    parser.state.id(),
                    parser.current_symbol_as_bytestring(),
                    parser.current_span(),
                )
            }),
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: self.current().to_owned(),
                    },
                    Severity::Error,
                    self.current_span(),
                );

                SimpleIdentifier::new(self.state.id(), ByteString::empty(), self.current_span())
            }
        }
    }

    pub fn parse_identifier_maybe_reserved(&mut self) -> SimpleIdentifier {
        if self.is_reserved_identifier(self.current_kind()) {
            self.next_but_first(|parser| {
                SimpleIdentifier::new(
                    parser.state.id(),
                    parser.current_symbol_as_bytestring(),
                    parser.current_span(),
                )
            })
        } else {
            self.parse_identifier()
        }
    }

    pub fn is_identifier_maybe_soft_reserved(&self, kind: TokenKind) -> bool {
        if let TokenKind::Identifier = kind {
            return true;
        }

        self.is_soft_reserved_identifier(kind)
    }

    pub fn is_identifier_maybe_reserved(&self, kind: TokenKind) -> bool {
        if let TokenKind::Identifier = kind {
            return true;
        }

        self.is_reserved_identifier(kind)
    }

    pub fn is_soft_reserved_identifier(&self, kind: TokenKind) -> bool {
        matches!(kind, |TokenKind::Parent| TokenKind::Self_
            | TokenKind::True
            | TokenKind::False
            | TokenKind::List
            | TokenKind::Null
            | TokenKind::Enum
            | TokenKind::From
            | TokenKind::Readonly)
    }

    pub fn is_reserved_identifier(&self, kind: TokenKind) -> bool {
        if self.is_soft_reserved_identifier(kind) {
            return true;
        }

        matches!(
            kind,
            TokenKind::Static
                | TokenKind::Abstract
                | TokenKind::Final
                | TokenKind::For
                | TokenKind::Private
                | TokenKind::Protected
                | TokenKind::Public
                | TokenKind::Include
                | TokenKind::IncludeOnce
                | TokenKind::Eval
                | TokenKind::Require
                | TokenKind::RequireOnce
                | TokenKind::LogicalOr
                | TokenKind::LogicalXor
                | TokenKind::LogicalAnd
                | TokenKind::Instanceof
                | TokenKind::New
                | TokenKind::Clone
                | TokenKind::Exit
                | TokenKind::Die
                | TokenKind::If
                | TokenKind::ElseIf
                | TokenKind::Else
                | TokenKind::EndIf
                | TokenKind::Echo
                | TokenKind::Do
                | TokenKind::While
                | TokenKind::EndWhile
                | TokenKind::EndFor
                | TokenKind::Foreach
                | TokenKind::EndForeach
                | TokenKind::Declare
                | TokenKind::EndDeclare
                | TokenKind::As
                | TokenKind::Try
                | TokenKind::Catch
                | TokenKind::Finally
                | TokenKind::Throw
                | TokenKind::Use
                | TokenKind::Insteadof
                | TokenKind::Global
                | TokenKind::Var
                | TokenKind::Unset
                | TokenKind::Isset
                | TokenKind::Empty
                | TokenKind::Continue
                | TokenKind::Goto
                | TokenKind::Function
                | TokenKind::Const
                | TokenKind::Return
                | TokenKind::Print
                | TokenKind::Yield
                | TokenKind::List
                | TokenKind::Switch
                | TokenKind::EndSwitch
                | TokenKind::Case
                | TokenKind::Default
                | TokenKind::Break
                | TokenKind::Array
                | TokenKind::Callable
                | TokenKind::Extends
                | TokenKind::Implements
                | TokenKind::Namespace
                | TokenKind::Trait
                | TokenKind::Interface
                | TokenKind::Class
                | TokenKind::ClassConstant
                | TokenKind::TraitConstant
                | TokenKind::FunctionConstant
                | TokenKind::MethodConstant
                | TokenKind::LineConstant
                | TokenKind::FileConstant
                | TokenKind::DirConstant
                | TokenKind::NamespaceConstant
                | TokenKind::HaltCompiler
                | TokenKind::CompilerHaltOffsetConstant
                | TokenKind::Fn
                | TokenKind::Match
        )
    }
}
