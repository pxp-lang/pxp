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
    StandaloneTypeUsedInIntersectionType,
    NestedDisjunctiveNormalFormType,
    InvalidBackedEnumType,
    UnitEnumsCannotHaveCaseValues,
    BackedEnumCaseMustHaveValue,
    CannotUseReservedKeywordAsTypeName,
    CannotUseReservedKeywordAsLabel,
    CannotUseReservedKeywordAsConstantName,
    InvalidClassModifier,
    InvalidMethodModifier,
    InvalidPropertyModifier,
    InvalidConstantModifier,
    CannotUseFinalWithAbstract,
    CannotUseFinalWithPrivateOnConstant,
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
            DiagnosticKind::StandaloneTypeUsedInIntersectionType => write!(f, "cannot use standalone type in intersection type"),
            DiagnosticKind::NestedDisjunctiveNormalFormType => write!(f, "DNF types cannot be nested"),
            DiagnosticKind::InvalidBackedEnumType => write!(f, "invalid backed enum type, must be `string` or `int`"),
            DiagnosticKind::UnitEnumsCannotHaveCaseValues => write!(f, "unit enums cannot have case values"),
            DiagnosticKind::BackedEnumCaseMustHaveValue => write!(f, "backed enum case must have value"),
            DiagnosticKind::CannotUseReservedKeywordAsTypeName => write!(f, "cannot use reserved keyword as type name"),
            DiagnosticKind::CannotUseReservedKeywordAsLabel => write!(f, "cannot use reserved keyword as label"),
            DiagnosticKind::CannotUseReservedKeywordAsConstantName => write!(f, "cannot use reserved keyword as constant name"),
            DiagnosticKind::InvalidClassModifier => write!(f, "invalid class modifier"),
            DiagnosticKind::InvalidMethodModifier => write!(f, "invalid method modifier"),
            DiagnosticKind::InvalidPropertyModifier => write!(f, "invalid property modifier"),
            DiagnosticKind::InvalidConstantModifier => write!(f, "invalid constant modifier"),
            DiagnosticKind::CannotUseFinalWithAbstract => write!(f, "cannot use final and abstract together"),
            DiagnosticKind::CannotUseFinalWithPrivateOnConstant => write!(f, "private constant cannot be final as it is not visible to other classes"),
            DiagnosticKind::UnexpectedEndOfFile => write!(f, "unexpected end of file"),
        }
    }
}