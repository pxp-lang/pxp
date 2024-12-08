use crate::{state::State, Parser, ParserDiagnostic};
use pxp_ast::*;
use pxp_bytestring::ByteString;
use pxp_diagnostics::Severity;

use pxp_span::Span;
use pxp_token::{Token, TokenKind};

impl<'a> Parser<'a> {
    /// Expect an unqualified identifier such as Foo or Bar for a class, interface, trait, or an enum name.
    pub fn type_identifier(state: &mut State) -> SimpleIdentifier {
        let current = state.current();
        match &current.kind {
            TokenKind::Identifier => {
                state.next();

                let symbol = current.symbol.to_bytestring();

                SimpleIdentifier::new(state.id(), symbol, current.span)
            }
            TokenKind::Enum | TokenKind::From => {
                state.next();

                let symbol = current.symbol.to_bytestring();

                SimpleIdentifier::new(state.id(), symbol, current.span)
            }
            TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
                state.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                    Severity::Error,
                    current.span,
                );

                state.next();

                let symbol = current.symbol.to_bytestring();

                SimpleIdentifier::new(state.id(), symbol, current.span)
            }
            t if is_reserved_identifier(t) => {
                state.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                    Severity::Error,
                    current.span,
                );

                state.next();

                let symbol = current.symbol.to_bytestring();

                SimpleIdentifier::new(state.id(), symbol, current.span)
            }
            _ => {
                state.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: current.to_owned(),
                    },
                    Severity::Error,
                    current.span,
                );

                SimpleIdentifier::new(state.id(), ByteString::empty(), current.span)
            }
        }
    }

    /// Expect an unqualified identifier such as foo or bar for a goto label name.
    pub fn label_identifier(state: &mut State) -> SimpleIdentifier {
        let current = state.current();
        match &current.kind {
            TokenKind::Identifier => {
                state.next();

                let symbol = current.symbol.to_bytestring();

                SimpleIdentifier::new(state.id(), symbol, current.span)
            }
            TokenKind::Enum | TokenKind::From => {
                state.next();

                let symbol = current.symbol.to_bytestring();

                SimpleIdentifier::new(state.id(), symbol, current.span)
            }
            TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
                state.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsLabel,
                    Severity::Error,
                    current.span,
                );

                state.next();

                let symbol = current.symbol.to_bytestring();

                SimpleIdentifier::new(state.id(), symbol, current.span)
            }
            t if is_reserved_identifier(t) => {
                state.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsLabel,
                    Severity::Error,
                    current.span,
                );

                state.next();

                let symbol = current.symbol.to_bytestring();

                SimpleIdentifier::new(state.id(), symbol, current.span)
            }
            _ => {
                state.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: current.to_owned(),
                    },
                    Severity::Error,
                    current.span,
                );

                SimpleIdentifier::new(state.id(), ByteString::empty(), current.span)
            }
        }
    }

    /// Expect an unqualified identifier such as Foo or Bar.
    pub fn identifier(state: &mut State) -> SimpleIdentifier {
        let current = state.current();
        if let TokenKind::Identifier = &current.kind {
            state.next();

            let symbol = current.symbol.to_bytestring();

            SimpleIdentifier::new(state.id(), symbol, current.span)
        } else {
            let previous = state.previous();

            state.diagnostic(
                ParserDiagnostic::UnexpectedToken {
                    token: current.to_owned(),
                },
                Severity::Error,
                current.span,
            );

            // Because identifiers cannot contain spaces, we can assume that the next identifier starts
            // one byte after the previous token ends.
            SimpleIdentifier::new(
                state.id(),
                ByteString::empty(),
                Span::flat(previous.span.end + 1),
            )
        }
    }

    /// Expect an unqualified or qualified identifier such as Foo, Bar or Foo\Bar.
    pub fn name(state: &mut State) -> SimpleIdentifier {
        let name = match state.current().kind {
            TokenKind::Identifier | TokenKind::QualifiedIdentifier => state.current().clone(),
            _ => {
                let span = state.current().span;

                state.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier, TokenKind::QualifiedIdentifier],
                        found: state.current().to_owned(),
                    },
                    Severity::Error,
                    span,
                );

                Token::missing(state.current().span)
            }
        };

        state.next();

        SimpleIdentifier::new(state.id(), name.symbol.to_bytestring(), name.span)
    }

    /// Expect an optional unqualified or qualified identifier such as Foo, Bar or Foo\Bar.
    pub fn optional_name(state: &mut State) -> Option<SimpleIdentifier> {
        let current = state.current();

        match &current.kind {
            TokenKind::Identifier | TokenKind::QualifiedIdentifier => {
                state.next();

                let symbol = current.symbol.to_bytestring();

                Some(SimpleIdentifier::new(state.id(), symbol, current.span))
            }
            t if is_reserved_identifier(t) => {
                state.next();

                let symbol = current.symbol.to_bytestring();

                Some(SimpleIdentifier::new(state.id(), symbol, current.span))
            }
            _ => None,
        }
    }

    /// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar.
    pub fn full_name(state: &mut State) -> SimpleIdentifier {
        let current = state.current();
        match &current.kind {
            TokenKind::Identifier
            | TokenKind::QualifiedIdentifier
            | TokenKind::FullyQualifiedIdentifier => {
                state.next();

                let symbol = current.symbol.to_bytestring();

                SimpleIdentifier::new(state.id(), symbol, current.span)
            }
            _ => {
                state.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: current.to_owned(),
                    },
                    Severity::Error,
                    current.span,
                );

                SimpleIdentifier::new(state.id(), ByteString::empty(), current.span)
            }
        }
    }

    /// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar.
    pub fn full_type_name(state: &mut State) -> SimpleIdentifier {
        let current = state.current();
        match &current.kind {
            TokenKind::Identifier
            | TokenKind::QualifiedIdentifier
            | TokenKind::FullyQualifiedIdentifier => {
                state.next();

                let symbol = current.symbol.to_bytestring();

                SimpleIdentifier::new(state.id(), symbol, current.span)
            }
            TokenKind::Enum | TokenKind::From => {
                state.next();

                let symbol = current.symbol.to_bytestring();

                SimpleIdentifier::new(state.id(), symbol, current.span)
            }
            TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
                state.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                    Severity::Error,
                    current.span,
                );

                state.next();

                let symbol = current.symbol.to_bytestring();

                SimpleIdentifier::new(state.id(), symbol, current.span)
            }
            t if is_reserved_identifier(t) => {
                state.diagnostic(
                    ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                    Severity::Error,
                    current.span,
                );

                state.next();

                let symbol = current.symbol.to_bytestring();

                SimpleIdentifier::new(state.id(), symbol, current.span)
            }
            _ => {
                state.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: current.to_owned(),
                    },
                    Severity::Error,
                    current.span,
                );

                SimpleIdentifier::new(state.id(), ByteString::empty(), current.span)
            }
        }
    }

    pub fn identifier_maybe_reserved(state: &mut State) -> SimpleIdentifier {
        let current = state.current();

        if is_reserved_identifier(&current.kind) {
            state.next();

            let symbol = current.symbol.to_bytestring();

            SimpleIdentifier::new(state.id(), symbol, current.span)
        } else {
            identifier(state)
        }
    }

    pub fn is_identifier_maybe_soft_reserved(kind: &TokenKind) -> bool {
        if let TokenKind::Identifier = kind {
            return true;
        }

        is_soft_reserved_identifier(kind)
    }

    pub fn is_identifier_maybe_reserved(kind: &TokenKind) -> bool {
        if let TokenKind::Identifier = kind {
            return true;
        }

        is_reserved_identifier(kind)
    }

    pub fn is_soft_reserved_identifier(kind: &TokenKind) -> bool {
        matches!(kind, |TokenKind::Parent| TokenKind::Self_
            | TokenKind::True
            | TokenKind::False
            | TokenKind::List
            | TokenKind::Null
            | TokenKind::Enum
            | TokenKind::From
            | TokenKind::Readonly)
    }

    pub fn is_reserved_identifier(kind: &TokenKind) -> bool {
        if is_soft_reserved_identifier(kind) {
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
