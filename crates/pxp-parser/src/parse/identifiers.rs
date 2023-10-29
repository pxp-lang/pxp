use pxp_ast::SimpleIdentifier;
use pxp_token::TokenKind;

use crate::state::ParserState;

pub fn optional_name(state: &mut ParserState) -> Option<SimpleIdentifier> {
    let current = state.stream.current();

    match &current.kind {
        TokenKind::Identifier | TokenKind::QualifiedIdentifier => {
            state.stream.next();

            Some(SimpleIdentifier {
                span: current.span,
                value: current.clone()
            })
        }
        t if is_reserved_identifier(t) => {
            state.stream.next();

            Some(SimpleIdentifier {
                span: current.span,
                value: current.clone()
            })
        }
        _ => None,
    }
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
            | TokenKind::Extends
            | TokenKind::Implements
            | TokenKind::Namespace
            | TokenKind::Trait
            | TokenKind::Interface
            | TokenKind::Class
            | TokenKind::__CLASS__
            | TokenKind::__TRAIT__
            | TokenKind::__FUNCTION__
            | TokenKind::__METHOD__
            | TokenKind::__LINE__
            | TokenKind::__FILE__
            | TokenKind::__DIR__
            | TokenKind::__NAMESPACE__
            | TokenKind::HaltCompiler
            | TokenKind::__COMPILER_HALT_OFFSET__
            | TokenKind::Fn
            | TokenKind::Match
    )
}

pub fn is_soft_reserved_identifier(kind: &TokenKind) -> bool {
    matches!(kind, TokenKind::True
        | TokenKind::False
        | TokenKind::List
        | TokenKind::Null
        | TokenKind::Enum
        | TokenKind::From
        | TokenKind::Readonly)
}