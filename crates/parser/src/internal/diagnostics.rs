use pxp_diagnostics::{Diagnostic, DiagnosticKind, Severity};
use pxp_span::Span;

use crate::Parser;

use std::fmt::Display;

use pxp_token::{OwnedToken, TokenKind};

#[derive(Debug, Clone)]
pub enum ParserDiagnostic {
    UnexpectedToken {
        token: OwnedToken,
    },
    ExpectedToken {
        expected: Vec<TokenKind>,
        found: OwnedToken,
    },
    ExpectedTokenExFound {
        expected: Vec<TokenKind>,
    },
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
    InvalidPropertyHook,
    ExpectedPropertyHook,
    CannotUseFinalWithAbstract,
    CannotUseFinalWithPrivateOnConstant,
    DuplicateModifier,
    MultipleVisibilityModifiers,
    MultipleSetVisibilityModifiers,
    CannotMixBracketedAndUnbracketedNamespaceDeclarations,
    NestedNamespace,
    PromotedPropertyCannotBeVariadic,
    ForbiddenTypeUsedInProperty,
    ReadonlyPropertyMustHaveType,
    CannotUsePositionalArgumentAfterNamedArgument,
    PositionalArgumentsOnly,
    OnlyAllowedOneArgument,
    ArgumentRequired,
    StaticPropertyCannotBeReadonly,
    ReadonlyPropertyCannotHaveDefaultValue,
    TryMustHaveCatchOrFinally,
    DynamicVariableNotAllowed,
    UnexpectedEndOfFile,
    UnexpectedEndOfFileExpected {
        expected: Vec<TokenKind>,
    },
    MixedImportTypes,
    InterfaceCannotUseTraits,
    InterfaceCannotContainConcreteMethods,
    InterfaceMembersMustBePublic,
}

impl DiagnosticKind for ParserDiagnostic {
    fn code(&self) -> &str {
        match self {
            ParserDiagnostic::UnexpectedToken { .. } => "P001",
            ParserDiagnostic::ExpectedToken { .. } => "P002",
            ParserDiagnostic::ExpectedTokenExFound { .. } => "P003",
            ParserDiagnostic::InvalidSpreadOperator => "P004",
            ParserDiagnostic::InvalidTargetForAttributes => "P005",
            ParserDiagnostic::CannotMixKeyedAndUnkeyedListEntries => "P006",
            ParserDiagnostic::AbstractMethodInNonAbstractClass => "P007",
            ParserDiagnostic::CannotHaveMultipleDefaultArmsInMatch => "P008",
            ParserDiagnostic::MissingType => "P009",
            ParserDiagnostic::StandaloneTypeUsedInNullableType => "P010",
            ParserDiagnostic::StandaloneTypeUsedInUnionType => "P011",
            ParserDiagnostic::StandaloneTypeUsedInIntersectionType => "P012",
            ParserDiagnostic::NestedDisjunctiveNormalFormType => "P013",
            ParserDiagnostic::InvalidBackedEnumType => "P014",
            ParserDiagnostic::UnitEnumsCannotHaveCaseValues => "P015",
            ParserDiagnostic::BackedEnumCaseMustHaveValue => "P016",
            ParserDiagnostic::CannotUseReservedKeywordAsTypeName => "P017",
            ParserDiagnostic::CannotUseReservedKeywordAsLabel => "P018",
            ParserDiagnostic::CannotUseReservedKeywordAsConstantName => "P019",
            ParserDiagnostic::InvalidClassModifier => "P020",
            ParserDiagnostic::InvalidMethodModifier => "P021",
            ParserDiagnostic::InvalidPropertyModifier => "P022",
            ParserDiagnostic::InvalidConstantModifier => "P023",
            ParserDiagnostic::InvalidPropertyHook => "P024",
            ParserDiagnostic::ExpectedPropertyHook => "P025",
            ParserDiagnostic::CannotUseFinalWithAbstract => "P026",
            ParserDiagnostic::CannotUseFinalWithPrivateOnConstant => "P027",
            ParserDiagnostic::DuplicateModifier => "P028",
            ParserDiagnostic::MultipleVisibilityModifiers => "P029",
            ParserDiagnostic::MultipleSetVisibilityModifiers => "P030",
            ParserDiagnostic::CannotMixBracketedAndUnbracketedNamespaceDeclarations => "P031",
            ParserDiagnostic::NestedNamespace => "P032",
            ParserDiagnostic::PromotedPropertyCannotBeVariadic => "P033",
            ParserDiagnostic::ForbiddenTypeUsedInProperty => "P034",
            ParserDiagnostic::ReadonlyPropertyMustHaveType => "P035",
            ParserDiagnostic::CannotUsePositionalArgumentAfterNamedArgument => "P036",
            ParserDiagnostic::PositionalArgumentsOnly => "P037",
            ParserDiagnostic::OnlyAllowedOneArgument => "P038",
            ParserDiagnostic::ArgumentRequired => "P039",
            ParserDiagnostic::StaticPropertyCannotBeReadonly => "P040",
            ParserDiagnostic::ReadonlyPropertyCannotHaveDefaultValue => "P041",
            ParserDiagnostic::TryMustHaveCatchOrFinally => "P042",
            ParserDiagnostic::DynamicVariableNotAllowed => "P043",
            ParserDiagnostic::UnexpectedEndOfFile => "P044",
            ParserDiagnostic::UnexpectedEndOfFileExpected { .. } => "P045",
            ParserDiagnostic::MixedImportTypes => "P046",
            ParserDiagnostic::InterfaceCannotUseTraits => "P049",
            ParserDiagnostic::InterfaceCannotContainConcreteMethods => "P050",
            ParserDiagnostic::InterfaceMembersMustBePublic => "P051",
        }
    }

    fn identifier(&self) -> &str {
        match self {
            ParserDiagnostic::UnexpectedToken { .. } => "parser.unexpected-token",
            ParserDiagnostic::ExpectedToken { .. } => "parser.expected-token",
            ParserDiagnostic::ExpectedTokenExFound { .. } => "parser.expected-token",
            ParserDiagnostic::InvalidSpreadOperator => "parser.invalid-spread-operator",
            ParserDiagnostic::InvalidTargetForAttributes => "parser.invalid-target-for-attributes",
            ParserDiagnostic::CannotMixKeyedAndUnkeyedListEntries => {
                "parser.cannot-mix-keyed-and-unkeyed-list-entries"
            }
            ParserDiagnostic::AbstractMethodInNonAbstractClass => {
                "parser.abstract-method-in-non-abstract-class"
            }
            ParserDiagnostic::CannotHaveMultipleDefaultArmsInMatch => {
                "parser.cannot-have-multiple-default-arms-in-match"
            }
            ParserDiagnostic::MissingType => "parser.missing-type",
            ParserDiagnostic::StandaloneTypeUsedInNullableType => {
                "parser.standalone-type-used-in-nullable-type"
            }
            ParserDiagnostic::StandaloneTypeUsedInUnionType => {
                "parser.standalone-type-used-in-union-type"
            }
            ParserDiagnostic::StandaloneTypeUsedInIntersectionType => {
                "parser.standalone-type-used-in-intersection-type"
            }
            ParserDiagnostic::NestedDisjunctiveNormalFormType => {
                "parser.nested-disjunctive-normal-form-type"
            }
            ParserDiagnostic::InvalidBackedEnumType => "parser.invalid-backed-enum-type",
            ParserDiagnostic::UnitEnumsCannotHaveCaseValues => "parser.unit-enums-cannot-have-case-values",
            ParserDiagnostic::BackedEnumCaseMustHaveValue => "parser.backed-enum-case-must-have-value",
            ParserDiagnostic::CannotUseReservedKeywordAsTypeName => {
                "parser.cannot-use-reserved-keyword-as-type-name"
            }
            ParserDiagnostic::CannotUseReservedKeywordAsLabel => {
                "parser.cannot-use-reserved-keyword-as-label"
            }
            ParserDiagnostic::CannotUseReservedKeywordAsConstantName => {
                "parser.cannot-use-reserved-keyword-as-constant-name"
            }
            ParserDiagnostic::InvalidClassModifier => "parser.invalid-class-modifier",
            ParserDiagnostic::InvalidMethodModifier => "parser.invalid-method-modifier",
            ParserDiagnostic::InvalidPropertyModifier => "parser.invalid-property-modifier",
            ParserDiagnostic::InvalidConstantModifier => "parser.invalid-constant-modifier",
            ParserDiagnostic::InvalidPropertyHook => "parser.invalid-property-hook",
            ParserDiagnostic::ExpectedPropertyHook => "parser.expected-property-hook",
            ParserDiagnostic::CannotUseFinalWithAbstract => "parser.cannot-use-final-with-abstract",
            ParserDiagnostic::CannotUseFinalWithPrivateOnConstant => {
                "parser.cannot-use-final-with-private-on-constant"
            }
            ParserDiagnostic::DuplicateModifier => "parser.duplicate-modifier",
            ParserDiagnostic::MultipleVisibilityModifiers => "parser.multiple-visibility-modifiers",
            ParserDiagnostic::MultipleSetVisibilityModifiers => "parser.multiple-set-visibility-modifiers",
            ParserDiagnostic::CannotMixBracketedAndUnbracketedNamespaceDeclarations => {
                "parser.cannot-mix-bracketed-and-unbracketed-namespace-declarations"
            }
            ParserDiagnostic::NestedNamespace => "parser.nested-namespace",
            ParserDiagnostic::PromotedPropertyCannotBeVariadic => "parser.promoted-property-cannot-be-variadic",
            ParserDiagnostic::ForbiddenTypeUsedInProperty => "parser.forbidden-type-used-in-property",
            ParserDiagnostic::ReadonlyPropertyMustHaveType => "parser.readonly-property-must-have-type",
            ParserDiagnostic::CannotUsePositionalArgumentAfterNamedArgument => {
                "parser.cannot-use-positional-argument-after-named-argument"
            }
            ParserDiagnostic::PositionalArgumentsOnly => "parser.positional-arguments-only",
            ParserDiagnostic::OnlyAllowedOneArgument => "parser.only-allowed-one-argument",
            ParserDiagnostic::ArgumentRequired => "parser.argument-required",
            ParserDiagnostic::StaticPropertyCannotBeReadonly => "parser.static-property-cannot-be-readonly",
            ParserDiagnostic::ReadonlyPropertyCannotHaveDefaultValue => "parser.readonly-property-cannot-have-default-value",
            ParserDiagnostic::TryMustHaveCatchOrFinally => "parser.try-must-have-catch-or-finally",
            ParserDiagnostic::DynamicVariableNotAllowed => "parser.dynamic-variable-not-allowed",
            ParserDiagnostic::UnexpectedEndOfFile => "parser.unexpected-end-of-file",
            ParserDiagnostic::UnexpectedEndOfFileExpected { .. } => "parser.unexpected-end-of-file-expected",
            ParserDiagnostic::MixedImportTypes => "parser.mixed-import-types",
            ParserDiagnostic::InterfaceCannotUseTraits => "parser.interface-cannot-use-traits",
            ParserDiagnostic::InterfaceCannotContainConcreteMethods => "parser.interface-cannot-contain-concrete-methods",
            ParserDiagnostic::InterfaceMembersMustBePublic => "parser.interface-members-must-be-public",
        }
    }
}

impl Display for ParserDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserDiagnostic::InterfaceCannotUseTraits => {
                write!(f, "interfaces cannot use traits")
            }
            ParserDiagnostic::InterfaceCannotContainConcreteMethods => {
                write!(f, "interfaces cannot contain concrete methods")
            }
            ParserDiagnostic::InterfaceMembersMustBePublic => {
                write!(f, "interface members must be public")
            }
            ParserDiagnostic::MultipleSetVisibilityModifiers => {
                write!(f, "cannot have multiple write / set visibility modifiers")
            }
            ParserDiagnostic::InvalidPropertyHook => {
                write!(f, "invalid property hook, expecting `get` or `set`")
            }
            ParserDiagnostic::ExpectedPropertyHook => write!(f, "expected a property hook"),
            ParserDiagnostic::UnexpectedToken { token } => {
                write!(f, "unexpected token {}", token.kind)
            }
            ParserDiagnostic::ExpectedToken { expected, found } => {
                if expected.len() == 1 {
                    write!(
                        f,
                        "unexpected token {}, expected {}",
                        found.kind,
                        expected.first().unwrap()
                    )
                } else {
                    write!(
                        f,
                        "unexpected token {}, expected one of {}",
                        found.kind,
                        expected
                            .iter()
                            .map(|kind| format!("{}", kind))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            ParserDiagnostic::ExpectedTokenExFound { expected } => {
                if expected.len() == 1 {
                    write!(f, "expected {}", expected.first().unwrap())
                } else {
                    write!(
                        f,
                        "expected one of {}",
                        expected
                            .iter()
                            .map(|kind| format!("{}", kind))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            ParserDiagnostic::InvalidSpreadOperator => write!(f, "cannot use spread operator here"),
            ParserDiagnostic::InvalidTargetForAttributes => {
                write!(f, "invalid target for attributes")
            }
            ParserDiagnostic::CannotMixKeyedAndUnkeyedListEntries => {
                write!(f, "cannot mix keyed and unkeyed list entries")
            }
            ParserDiagnostic::AbstractMethodInNonAbstractClass => {
                write!(f, "cannot declare abstract method in non-abstract class")
            }
            ParserDiagnostic::CannotHaveMultipleDefaultArmsInMatch => {
                write!(f, "cannot have multiple default arms in match")
            }
            ParserDiagnostic::MissingType => write!(f, "missing type"),
            ParserDiagnostic::StandaloneTypeUsedInNullableType => {
                write!(f, "cannot use standalone type in nullable type")
            }
            ParserDiagnostic::StandaloneTypeUsedInUnionType => {
                write!(f, "cannot use standalone type in union type")
            }
            ParserDiagnostic::StandaloneTypeUsedInIntersectionType => {
                write!(f, "cannot use standalone type in intersection type")
            }
            ParserDiagnostic::NestedDisjunctiveNormalFormType => {
                write!(f, "DNF types cannot be nested")
            }
            ParserDiagnostic::InvalidBackedEnumType => {
                write!(f, "invalid backed enum type, must be `string` or `int`")
            }
            ParserDiagnostic::UnitEnumsCannotHaveCaseValues => {
                write!(f, "unit enums cannot have case values")
            }
            ParserDiagnostic::BackedEnumCaseMustHaveValue => {
                write!(f, "backed enum case must have value")
            }
            ParserDiagnostic::CannotUseReservedKeywordAsTypeName => {
                write!(f, "cannot use reserved keyword as type name")
            }
            ParserDiagnostic::CannotUseReservedKeywordAsLabel => {
                write!(f, "cannot use reserved keyword as label")
            }
            ParserDiagnostic::CannotUseReservedKeywordAsConstantName => {
                write!(f, "cannot use reserved keyword as constant name")
            }
            ParserDiagnostic::InvalidClassModifier => write!(f, "invalid class modifier"),
            ParserDiagnostic::InvalidMethodModifier => write!(f, "invalid method modifier"),
            ParserDiagnostic::InvalidPropertyModifier => write!(f, "invalid property modifier"),
            ParserDiagnostic::InvalidConstantModifier => write!(f, "invalid constant modifier"),
            ParserDiagnostic::CannotUseFinalWithAbstract => {
                write!(f, "cannot use final and abstract together")
            }
            ParserDiagnostic::CannotUseFinalWithPrivateOnConstant => write!(
                f,
                "private constant cannot be final as it is not visible to other classes"
            ),
            ParserDiagnostic::DuplicateModifier => write!(f, "duplicate modifier"),
            ParserDiagnostic::MultipleVisibilityModifiers => {
                write!(f, "cannot have multiple visibility modifiers")
            }
            ParserDiagnostic::CannotMixBracketedAndUnbracketedNamespaceDeclarations => write!(
                f,
                "cannot mix bracketed and unbracketed namespace declarations"
            ),
            ParserDiagnostic::NestedNamespace => write!(f, "cannot nest namespaces"),
            ParserDiagnostic::PromotedPropertyCannotBeVariadic => {
                write!(f, "promoted property cannot be variadic")
            }
            ParserDiagnostic::ForbiddenTypeUsedInProperty => {
                write!(f, "forbidden type used in property")
            }
            ParserDiagnostic::ReadonlyPropertyMustHaveType => {
                write!(f, "readonly property must have type")
            }
            ParserDiagnostic::CannotUsePositionalArgumentAfterNamedArgument => {
                write!(f, "cannot use positional argument after named argument")
            }
            ParserDiagnostic::PositionalArgumentsOnly => {
                write!(f, "only positional arguments are allowed")
            }
            ParserDiagnostic::OnlyAllowedOneArgument => write!(f, "only one argument is allowed"),
            ParserDiagnostic::ArgumentRequired => write!(f, "argument required"),
            ParserDiagnostic::StaticPropertyCannotBeReadonly => {
                write!(f, "static property cannot be readonly")
            }
            ParserDiagnostic::ReadonlyPropertyCannotHaveDefaultValue => {
                write!(f, "readonly property cannot have default value")
            }
            ParserDiagnostic::TryMustHaveCatchOrFinally => {
                write!(f, "try must have catch or finally")
            }
            ParserDiagnostic::DynamicVariableNotAllowed => {
                write!(f, "dynamic variable not allowed")
            }
            ParserDiagnostic::UnexpectedEndOfFile => write!(f, "unexpected end of file"),
            ParserDiagnostic::UnexpectedEndOfFileExpected { expected } => {
                if expected.len() == 1 {
                    write!(
                        f,
                        "unexpected end of file, expected {}",
                        expected.first().unwrap()
                    )
                } else {
                    write!(
                        f,
                        "unexpected end of file, expected one of {}",
                        expected
                            .iter()
                            .map(|kind| format!("{}", kind))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            ParserDiagnostic::MixedImportTypes => write!(f, "cannot mix import types"),
        }
    }
}

impl<'a> Parser<'a> {
    pub(crate) fn diagnostic(
        &mut self,
        diagnostic: ParserDiagnostic,
        severity: Severity,
        span: Span,
    ) {
        self.diagnostics
            .push(Diagnostic::new(diagnostic, severity, span));
    }
}
