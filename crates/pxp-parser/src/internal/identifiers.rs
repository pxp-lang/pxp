use crate::{state::State, ParserDiagnostic};
use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_symbol::Symbol;
use pxp_token::{Token, TokenKind};

/// Expect an unqualified identifier such as Foo or Bar for a class, interface, trait, or an enum name.
pub fn type_identifier(state: &mut State) -> SimpleIdentifier {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier => {
            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
        }
        TokenKind::Enum | TokenKind::From => {
            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
        }
        TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
            state.diagnostic(
                ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                Severity::Error,
                current.span,
            );

            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
        }
        t if is_reserved_identifier(t) => {
            state.diagnostic(
                ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                Severity::Error,
                current.span,
            );

            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
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

            SimpleIdentifier::new(Symbol::missing(), current.span)
        }
    }
}

/// Expect an unqualified identifier such as foo or bar for a goto label name.
pub fn label_identifier(state: &mut State) -> SimpleIdentifier {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier => {
            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
        }
        TokenKind::Enum | TokenKind::From => {
            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
        }
        TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
            state.diagnostic(
                ParserDiagnostic::CannotUseReservedKeywordAsLabel,
                Severity::Error,
                current.span,
            );

            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
        }
        t if is_reserved_identifier(t) => {
            state.diagnostic(
                ParserDiagnostic::CannotUseReservedKeywordAsLabel,
                Severity::Error,
                current.span,
            );

            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
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

            SimpleIdentifier::new(Symbol::missing(), current.span)
        }
    }
}

/// Expect an unqualified identifier such as FOO or BAR for a constant name.
pub fn constant_identifier(state: &mut State) -> SimpleIdentifier {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier
        | TokenKind::Enum
        | TokenKind::From
        | TokenKind::Self_
        | TokenKind::Parent => {
            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
        }
        t if is_reserved_identifier(t) => {
            state.diagnostic(
                ParserDiagnostic::CannotUseReservedKeywordAsConstantName,
                Severity::Error,
                current.span,
            );

            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
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

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
        }
    }
}

/// Expect an unqualified identifier such as Foo or Bar.
pub fn identifier(state: &mut State) -> SimpleIdentifier {
    let current = state.stream.current();
    if let TokenKind::Identifier = &current.kind {
        state.stream.next();

        let symbol = current.symbol.unwrap();

        SimpleIdentifier::new(symbol, current.span)
    } else {
        state.diagnostic(
            ParserDiagnostic::UnexpectedToken { token: *current },
            Severity::Error,
            current.span,
        );

        SimpleIdentifier::new(Symbol::missing(), current.span)
    }
}

/// Expect an unqualified or qualified identifier such as Foo, Bar or Foo\Bar.
pub fn name(state: &mut State) -> SimpleIdentifier {
    let name = match state.stream.current().kind {
        TokenKind::Identifier | TokenKind::QualifiedIdentifier => *state.stream.current(),
        _ => {
            let span = state.stream.current().span;

            state.diagnostic(
                ParserDiagnostic::ExpectedToken { expected: vec![TokenKind::Identifier, TokenKind::QualifiedIdentifier], found: *state.stream.current() },
                Severity::Error,
                span,
            );

            Token::missing(state.stream.current().span)
        }
    };

    state.stream.next();

    SimpleIdentifier::new(name.symbol.unwrap(), name.span)
}

/// Expect an optional unqualified or qualified identifier such as Foo, Bar or Foo\Bar.
pub fn optional_name(state: &mut State) -> Option<SimpleIdentifier> {
    let current = state.stream.current();

    match &current.kind {
        TokenKind::Identifier | TokenKind::QualifiedIdentifier => {
            state.stream.next();

            let symbol = current.symbol.unwrap();

            Some(SimpleIdentifier::new(symbol, current.span))
        }
        t if is_reserved_identifier(t) => {
            state.stream.next();

            let symbol = current.symbol.unwrap();

            Some(SimpleIdentifier::new(symbol, current.span))
        }
        _ => None,
    }
}

/// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar.
pub fn full_name(state: &mut State) -> SimpleIdentifier {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier
        | TokenKind::QualifiedIdentifier
        | TokenKind::FullyQualifiedIdentifier => {
            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
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

            SimpleIdentifier::new(Symbol::missing(), current.span)
        }
    }
}

/// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar.
pub fn full_type_name(state: &mut State) -> SimpleIdentifier {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier
        | TokenKind::QualifiedIdentifier
        | TokenKind::FullyQualifiedIdentifier => {
            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
        }
        TokenKind::Enum | TokenKind::From => {
            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
        }
        TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
            state.diagnostic(
                ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                Severity::Error,
                current.span,
            );

            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
        }
        t if is_reserved_identifier(t) => {
            state.diagnostic(
                ParserDiagnostic::CannotUseReservedKeywordAsTypeName,
                Severity::Error,
                current.span,
            );

            state.stream.next();

            let symbol = current.symbol.unwrap();

            SimpleIdentifier::new(symbol, current.span)
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

            SimpleIdentifier::new(Symbol::missing(), current.span)
        }
    }
}

pub fn identifier_maybe_reserved(state: &mut State) -> SimpleIdentifier {
    let current = state.stream.current();

    if is_reserved_identifier(&current.kind) {
        state.stream.next();

        let symbol = current.symbol.unwrap();

        SimpleIdentifier::new(symbol, current.span)
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
