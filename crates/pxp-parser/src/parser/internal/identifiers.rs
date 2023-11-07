use crate::lexer::token::TokenKind;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::error;
use crate::parser::error::ParseResult;
use crate::parser::state::State;

use crate::peek_token;

pub fn identifier_of(state: &mut State, kinds: &[&str]) -> ParseResult<SimpleIdentifier> {
    let ident = identifier(state)?;

    let name = ident.value.to_string();

    if kinds.contains(&name.as_str()) {
        Ok(ident)
    } else {
        Err(error::unexpected_identifier(
            kinds.iter().map(|s| s.to_string()).collect(),
            name,
            ident.span,
        ))
    }
}

/// Expect an unqualified identifier such as Foo or Bar for a class, interface, trait, or an enum name.
pub fn type_identifier(state: &mut State) -> ParseResult<SimpleIdentifier> {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier => {
            let span = current.span;

            state.stream.next();

            Ok(SimpleIdentifier {
                span,
                value: current.value.clone(),
            })
        }
        TokenKind::Enum | TokenKind::From => {
            let span = current.span;
            let name = current.to_string().into();

            state.stream.next();

            Ok(SimpleIdentifier { span, value: name })
        }
        TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
            state.record(error::cannot_use_reserved_keyword_as_a_type_name(
                current.span,
                current.to_string(),
            ));

            let span = current.span;
            let name = current.to_string().into();

            state.stream.next();

            Ok(SimpleIdentifier { span, value: name })
        }
        t if is_reserved_identifier(t) => {
            state.record(error::cannot_use_reserved_keyword_as_a_type_name(
                current.span,
                current.to_string(),
            ));

            let span = current.span;
            let name = current.to_string().into();

            state.stream.next();

            Ok(SimpleIdentifier { span, value: name })
        }
        _ => Err(error::unexpected_token(
            vec!["an identifier".to_owned()],
            current,
        )),
    }
}

/// Expect an unqualified identifier such as foo or bar for a goto label name.
pub fn label_identifier(state: &mut State) -> ParseResult<SimpleIdentifier> {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier => {
            let span = current.span;

            state.stream.next();

            Ok(SimpleIdentifier {
                span,
                value: current.value.clone(),
            })
        }
        TokenKind::Enum | TokenKind::From => {
            let span = current.span;
            let name = current.to_string().into();

            state.stream.next();

            Ok(SimpleIdentifier { span, value: name })
        }
        TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
            // TODO(azjezz): re-evaluate whether this is a good idea
            state.record(error::cannot_use_reserved_keyword_as_a_goto_label(
                current.span,
                current.to_string(),
            ));

            let span = current.span;
            let name = current.to_string().into();

            state.stream.next();

            Ok(SimpleIdentifier { span, value: name })
        }
        t if is_reserved_identifier(t) => {
            state.record(error::cannot_use_reserved_keyword_as_a_goto_label(
                current.span,
                current.to_string(),
            ));

            let span = current.span;
            let name = current.to_string().into();

            state.stream.next();

            Ok(SimpleIdentifier { span, value: name })
        }
        _ => Err(error::unexpected_token(
            vec!["an identifier".to_owned()],
            current,
        )),
    }
}

/// Expect an unqualified identifier such as FOO or BAR for a constant name.
pub fn constant_identifier(state: &mut State) -> ParseResult<SimpleIdentifier> {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier => {
            let span = current.span;

            state.stream.next();

            Ok(SimpleIdentifier {
                span,
                value: current.value.clone(),
            })
        }
        TokenKind::Enum | TokenKind::From | TokenKind::Self_ | TokenKind::Parent => {
            let span = current.span;
            let name = current.to_string().into();

            state.stream.next();

            Ok(SimpleIdentifier { span, value: name })
        }
        t if is_reserved_identifier(t) => {
            state.record(error::cannot_use_reserved_keyword_as_a_constant_name(
                current.span,
                current.to_string(),
            ));

            let span = current.span;
            let name = current.to_string().into();

            state.stream.next();

            Ok(SimpleIdentifier { span, value: name })
        }
        _ => Err(error::unexpected_token(
            vec!["an identifier".to_owned()],
            current,
        )),
    }
}

/// Expect an unqualified identifier such as Foo or Bar.
pub fn identifier(state: &mut State) -> ParseResult<SimpleIdentifier> {
    let current = state.stream.current();
    if let TokenKind::Identifier = &current.kind {
        let span = current.span;

        state.stream.next();

        Ok(SimpleIdentifier {
            span,
            value: current.value.clone(),
        })
    } else {
        Err(error::unexpected_token(
            vec!["an identifier".to_owned()],
            current,
        ))
    }
}

/// Expect an unqualified or qualified identifier such as Foo, Bar or Foo\Bar.
pub fn name(state: &mut State) -> ParseResult<SimpleIdentifier> {
    let name = peek_token!([
        TokenKind::Identifier | TokenKind::QualifiedIdentifier => {
            state.stream.current().value.clone()
        },
    ], state, "an identifier");

    let span = state.stream.current().span;
    state.stream.next();

    Ok(SimpleIdentifier { span, value: name })
}

/// Expect an optional unqualified or qualified identifier such as Foo, Bar or Foo\Bar.
pub fn optional_name(state: &mut State) -> Option<SimpleIdentifier> {
    let current = state.stream.current();

    match &current.kind {
        TokenKind::Identifier | TokenKind::QualifiedIdentifier => {
            state.stream.next();

            Some(SimpleIdentifier {
                span: current.span,
                value: current.value.clone(),
            })
        }
        t if is_reserved_identifier(t) => {
            state.stream.next();

            Some(SimpleIdentifier {
                span: current.span,
                value: current.value.clone(),
            })
        }
        _ => None,
    }
}

/// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar.
pub fn full_name(state: &mut State) -> ParseResult<SimpleIdentifier> {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier
        | TokenKind::QualifiedIdentifier
        | TokenKind::FullyQualifiedIdentifier => {
            let span = current.span;

            state.stream.next();

            Ok(SimpleIdentifier {
                span,
                value: current.value.clone(),
            })
        }
        _ => Err(error::unexpected_token(
            vec!["an identifier".to_owned()],
            current,
        )),
    }
}

/// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar.
pub fn full_type_name(state: &mut State) -> ParseResult<SimpleIdentifier> {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier
        | TokenKind::QualifiedIdentifier
        | TokenKind::FullyQualifiedIdentifier => {
            let span = current.span;

            state.stream.next();

            Ok(SimpleIdentifier {
                span,
                value: current.value.clone(),
            })
        }
        TokenKind::Enum | TokenKind::From => {
            let span = current.span;
            let name = current.to_string().into();

            state.stream.next();

            Ok(SimpleIdentifier { span, value: name })
        }
        TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
            state.record(error::cannot_use_type_in_context(
                current.span,
                current.to_string(),
            ));

            let span = current.span;
            let name = current.to_string().into();

            state.stream.next();

            Ok(SimpleIdentifier { span, value: name })
        }
        t if is_reserved_identifier(t) => {
            state.record(error::cannot_use_reserved_keyword_as_a_type_name(
                current.span,
                current.to_string(),
            ));

            let span = current.span;
            let name = current.to_string().into();

            state.stream.next();

            Ok(SimpleIdentifier { span, value: name })
        }
        _ => Err(error::unexpected_token(
            vec!["an identifier".to_owned()],
            current,
        )),
    }
}

/// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar.
pub fn full_type_name_including_self(state: &mut State) -> ParseResult<SimpleIdentifier> {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier
        | TokenKind::QualifiedIdentifier
        | TokenKind::FullyQualifiedIdentifier => {
            let span = current.span;

            state.stream.next();

            Ok(SimpleIdentifier {
                span,
                value: current.value.clone(),
            })
        }
        TokenKind::Enum
        | TokenKind::From
        | TokenKind::Self_
        | TokenKind::Static
        | TokenKind::Parent => {
            let span = current.span;
            let name = current.to_string().into();

            state.stream.next();

            Ok(SimpleIdentifier { span, value: name })
        }
        t if is_reserved_identifier(t) => {
            state.record(error::cannot_use_reserved_keyword_as_a_type_name(
                current.span,
                current.to_string(),
            ));

            let span = current.span;
            let name = current.to_string().into();

            state.stream.next();

            Ok(SimpleIdentifier { span, value: name })
        }
        _ => Err(error::unexpected_token(
            vec!["an identifier".to_owned()],
            current,
        )),
    }
}

pub fn identifier_maybe_reserved(state: &mut State) -> ParseResult<SimpleIdentifier> {
    let current = state.stream.current();

    if is_reserved_identifier(&current.kind) {
        let name = current.to_string().into();
        let span = current.span;
        state.stream.next();

        Ok(SimpleIdentifier { span, value: name })
    } else {
        identifier(state)
    }
}

pub fn identifier_maybe_soft_reserved(state: &mut State) -> ParseResult<SimpleIdentifier> {
    let current = state.stream.current();

    if is_soft_reserved_identifier(&current.kind) {
        let name = current.to_string().into();
        let span = current.span;
        state.stream.next();

        Ok(SimpleIdentifier { span, value: name })
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
