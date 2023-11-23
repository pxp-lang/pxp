use crate::state::State;
use pxp_ast::identifiers::{SimpleIdentifier, Identifier};
use pxp_diagnostics::{DiagnosticKind, Severity};
use pxp_token::{TokenKind, Token};

use crate::peek_token;

pub fn identifier_of(state: &mut State, kinds: &[TokenKind]) -> SimpleIdentifier {
    let ident = identifier(state);

    if kinds.contains(&ident.token.kind) {
        ident
    } else {
        todo!("tolerant error handling: missing valid identifier")
    }
}

/// Expect an unqualified identifier such as Foo or Bar for a class, interface, trait, or an enum name.
pub fn type_identifier(state: &mut State) -> SimpleIdentifier {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier => {
            let span = current.span;

            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        TokenKind::Enum | TokenKind::From => {
            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
            // TODO: Report invalid keyword as type name
            // state.record(error::cannot_use_reserved_keyword_as_a_type_name(
            //     current.span,
            //     current.to_string(),
            // ));

            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        t if is_reserved_identifier(t) => {
            // TODO: Report invalid keyword as type name
            // state.record(error::cannot_use_reserved_keyword_as_a_type_name(
            //     current.span,
            //     current.to_string(),
            // ));

            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        _ => todo!("tolerant mode"), /*Err(error::unexpected_token(
                                         vec!["an identifier".to_owned()],
                                         current,
                                     ))*/
    }
}

/// Expect an unqualified identifier such as foo or bar for a goto label name.
pub fn label_identifier(state: &mut State) -> SimpleIdentifier {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier => {
            let span = current.span;

            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        TokenKind::Enum | TokenKind::From => {
            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
            // TODO: Report invalid keyword as type name
            // state.record(error::cannot_use_reserved_keyword_as_a_goto_label(
            //     current.span,
            //     current.to_string(),
            // ));

            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        t if is_reserved_identifier(t) => {
            // TODO: Report invalid keyword as type name
            // state.record(error::cannot_use_reserved_keyword_as_a_goto_label(
            //     current.span,
            //     current.to_string(),
            // ));

            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        _ => todo!("tolerant mode"), /*Err(error::unexpected_token(
                                         vec!["an identifier".to_owned()],
                                         current,
                                     ))*/
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
            let span = current.span;

            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        t if is_reserved_identifier(t) => {
            // TODO: Report invalid keyword as type name
            // state.record(error::cannot_use_reserved_keyword_as_a_constant_name(
            //     current.span,
            //     current.to_string(),
            // ));

            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        _ => todo!("tolerant mode"), /*Err(error::unexpected_token(
                                         vec!["an identifier".to_owned()],
                                         current,
                                     ))*/
    }
}

/// Expect an unqualified identifier such as Foo or Bar.
pub fn identifier(state: &mut State) -> SimpleIdentifier {
    let current = state.stream.current();
    if let TokenKind::Identifier = &current.kind {
        state.stream.next();

        SimpleIdentifier { token: *current }
    } else {
        state.diagnostic(
            DiagnosticKind::UnexpectedToken { token: *current },
            Severity::Error,
            current.span,
        );

        SimpleIdentifier::new(Token::missing(current.span))
    }
}

/// Expect an unqualified or qualified identifier such as Foo, Bar or Foo\Bar.
pub fn name(state: &mut State) -> SimpleIdentifier {
    let name = peek_token!([
        TokenKind::Identifier | TokenKind::QualifiedIdentifier => {
            state.stream.current()
        },
    ], state, "an identifier");

    let span = state.stream.current().span;
    state.stream.next();

    SimpleIdentifier { token: *name }
}

/// Expect an optional unqualified or qualified identifier such as Foo, Bar or Foo\Bar.
pub fn optional_name(state: &mut State) -> Option<SimpleIdentifier> {
    let current = state.stream.current();

    match &current.kind {
        TokenKind::Identifier | TokenKind::QualifiedIdentifier => {
            state.stream.next();

            Some(SimpleIdentifier { token: *current })
        }
        t if is_reserved_identifier(t) => {
            state.stream.next();

            Some(SimpleIdentifier { token: *current })
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
            let span = current.span;

            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        _ => todo!("tolerant mode"), /*Err(error::unexpected_token(
                                         vec!["an identifier".to_owned()],
                                         current,
                                     ))*/
    }
}

/// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar.
pub fn full_type_name(state: &mut State) -> SimpleIdentifier {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier
        | TokenKind::QualifiedIdentifier
        | TokenKind::FullyQualifiedIdentifier => {
            let span = current.span;

            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        TokenKind::Enum | TokenKind::From => {
            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
            // TODO: Report invalid keyword as type name
            // state.record(error::cannot_use_type_in_context(
            //     current.span,
            //     current.to_string(),
            // ));

            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        t if is_reserved_identifier(t) => {
            // // TODO: Report invalid keyword as type name
            // state.record(error::cannot_use_reserved_keyword_as_a_type_name(
            //     current.span,
            //     current.to_string(),
            // ));

            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        _ => todo!("tolerant mode"), /*Err(error::unexpected_token(
                                         vec!["an identifier".to_owned()],
                                         current,
                                     ))*/
    }
}

/// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar.
pub fn full_type_name_including_self(state: &mut State) -> SimpleIdentifier {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier
        | TokenKind::QualifiedIdentifier
        | TokenKind::FullyQualifiedIdentifier => {
            let span = current.span;

            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        TokenKind::Enum
        | TokenKind::From
        | TokenKind::Self_
        | TokenKind::Static
        | TokenKind::Parent => {
            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        t if is_reserved_identifier(t) => {
            // TODO: Report invalid keyword as type name
            // state.record(error::cannot_use_reserved_keyword_as_a_type_name(
            //     current.span,
            //     current.to_string(),
            // ));

            state.stream.next();

            SimpleIdentifier { token: *current }
        }
        _ => todo!("tolerant mode"), /*Err(error::unexpected_token(
                                         vec!["an identifier".to_owned()],
                                         current,
                                     ))*/
    }
}

pub fn identifier_maybe_reserved(state: &mut State) -> SimpleIdentifier {
    let current = state.stream.current();

    if is_reserved_identifier(&current.kind) {
        state.stream.next();

        SimpleIdentifier { token: *current }
    } else {
        identifier(state)
    }
}

pub fn identifier_maybe_soft_reserved(state: &mut State) -> SimpleIdentifier {
    let current = state.stream.current();

    if is_soft_reserved_identifier(&current.kind) {
        state.stream.next();

        SimpleIdentifier { token: *current }
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
