use crate::expressions;
use crate::internal::data_type;
use crate::internal::utils;
use crate::internal::variables;
use crate::state::State;
use crate::ParserDiagnostic;
use pxp_ast::*;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

pub fn parse(state: &mut State, modifiers: PropertyModifierGroup) -> Property {
    let ty = data_type::optional_data_type(state);

    let mut entries = vec![];
    let mut type_checked = false;
    loop {
        let variable = variables::simple_variable(state);

        if !type_checked {
            type_checked = true;
            if modifiers.has_readonly() && modifiers.has_static() {
                state.diagnostic(
                    ParserDiagnostic::StaticPropertyCannotBeReadonly,
                    Severity::Error,
                    state.current().span,
                );
            }

            match &ty {
                Some(ty) => {
                    if ty.includes_callable() || ty.is_bottom() {
                        state.diagnostic(
                            ParserDiagnostic::ForbiddenTypeUsedInProperty,
                            Severity::Error,
                            ty.get_span(),
                        );
                    }
                }
                None => {
                    if let Some(modifier) = modifiers.get_readonly() {
                        state.diagnostic(
                            ParserDiagnostic::ReadonlyPropertyMustHaveType,
                            Severity::Error,
                            modifier.span(),
                        );
                    }
                }
            }
        }

        let current = state.current();
        if current.kind == TokenKind::Equals {
            if let Some(modifier) = modifiers.get_readonly() {
                state.diagnostic(
                    ParserDiagnostic::ReadonlyPropertyCannotHaveDefaultValue,
                    Severity::Error,
                    modifier.span(),
                );
            }

            state.next();
            let value = expressions::create(state);
            let span = Span::combine(variable.span, value.span);

            entries.push(PropertyEntry {
                id: state.id(),
                span,
                kind: PropertyEntryKind::Initialized(InitializedPropertyEntry {
                    id: state.id(),
                    span,
                    variable,
                    equals: current.span,
                    value,
                }),
            });
        } else {
            entries.push(PropertyEntry {
                id: state.id(),
                span: variable.span,
                kind: PropertyEntryKind::Uninitialized(UninitializedPropertyEntry {
                    id: state.id(),
                    span: variable.span,
                    variable,
                }),
            });
        }

        if state.current().kind == TokenKind::Comma {
            state.next();
        } else {
            break;
        }
    }

    let end = utils::skip_semicolon(state);

    Property {
        id: state.id(),
        span: if ty.is_some() {
            Span::combine(ty.span(), end)
        } else {
            entries.span()
        },
        r#type: ty,
        modifiers,
        attributes: state.get_attributes(),
        entries,
        end,
    }
}

pub fn parse_var(state: &mut State) -> VariableProperty {
    utils::skip(state, TokenKind::Var);

    let ty = data_type::optional_data_type(state);

    let mut entries: Vec<PropertyEntry> = vec![];
    let mut type_checked = false;
    loop {
        let variable = variables::simple_variable(state);

        if !type_checked {
            type_checked = true;

            if let Some(ty) = &ty {
                if ty.includes_callable() || ty.is_bottom() {
                    state.diagnostic(
                        ParserDiagnostic::ForbiddenTypeUsedInProperty,
                        Severity::Error,
                        ty.get_span(),
                    );
                }
            }
        }

        let current = state.current();
        if current.kind == TokenKind::Equals {
            state.next();
            let value = expressions::create(state);
            let span = Span::combine(variable.span, value.span);

            entries.push(PropertyEntry {
                id: state.id(),
                span,
                kind: PropertyEntryKind::Initialized(InitializedPropertyEntry {
                    id: state.id(),
                    span,
                    variable,
                    equals: span,
                    value,
                }),
            });
        } else {
            entries.push(PropertyEntry {
                id: state.id(),
                span: variable.span,
                kind: PropertyEntryKind::Uninitialized(UninitializedPropertyEntry {
                    id: state.id(),
                    span: variable.span,
                    variable,
                }),
            });
        }

        if state.current().kind == TokenKind::Comma {
            state.next();
        } else {
            break;
        }
    }

    let end = utils::skip_semicolon(state);

    VariableProperty {
        id: state.id(),
        span: if ty.is_some() {
            Span::combine(ty.span(), end)
        } else {
            entries.span()
        },
        r#type: ty,
        attributes: state.get_attributes(),
        entries,
        end,
    }
}
