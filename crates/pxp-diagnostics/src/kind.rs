use std::fmt::Display;

use pxp_token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum DiagnosticKind {
    UnexpectedToken { token: Token },
    ExpectedToken { expected: Vec<TokenKind>, found: Token },
    InvalidSpreadOperator,
    InvalidTargetForAttributes,
    CannotMixKeyedAndUnkeyedListEntries,
    AbstractMethodInNonAbstractClass,
    CannotHaveMultipleDefaultArmsInMatch,
    MissingType,
    StandaloneTypeUsedInNullableType,
    StandaloneTypeUsedInUnionType,
    NestedDisjunctiveNormalFormType,
    UnexpectedEndOfFile,
}

impl Display for DiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticKind::UnexpectedToken { token } => write!(f, "unexpected token: {:?}", token),
            DiagnosticKind::ExpectedToken { expected, found } => if expected.len() == 1 {
                write!(f, "unexpected token {:?}, expected {}", found, expected.first().unwrap())
            } else {
                write!(f, "unexpected token {:?}, expected one of {}", found, expected.iter().map(|kind| format!("{}", kind)).collect::<Vec<_>>().join(", "))
            },
            DiagnosticKind::InvalidSpreadOperator => write!(f, "cannot use spread operator here"),
            DiagnosticKind::InvalidTargetForAttributes => write!(f, "invalid target for attributes"),
            DiagnosticKind::CannotMixKeyedAndUnkeyedListEntries => write!(f, "cannot mix keyed and unkeyed list entries"),
            DiagnosticKind::AbstractMethodInNonAbstractClass => write!(f, "cannot declare abstract method in non-abstract class"),
            DiagnosticKind::CannotHaveMultipleDefaultArmsInMatch => write!(f, "cannot have multiple default arms in match"),
            DiagnosticKind::MissingType => write!(f, "missing type"),
            DiagnosticKind::StandaloneTypeUsedInNullableType => write!(f, "cannot use standalone type in nullable type"),
            DiagnosticKind::StandaloneTypeUsedInUnionType => write!(f, "cannot use standalone type in union type"),
            DiagnosticKind::NestedDisjunctiveNormalFormType => write!(f, "DNF types cannot be nested"),
            DiagnosticKind::UnexpectedEndOfFile => write!(f, "unexpected end of file"),
        }
    }
}