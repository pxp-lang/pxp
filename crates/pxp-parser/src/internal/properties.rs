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
                    state.stream.current().span,
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

        let current = state.stream.current();
        if current.kind == TokenKind::Equals {
            if let Some(modifier) = modifiers.get_readonly() {
                state.diagnostic(
                    ParserDiagnostic::ReadonlyPropertyCannotHaveDefaultValue,
                    Severity::Error,
                    modifier.span(),
                );
            }

            state.stream.next();
            let value = expressions::create(state);

            entries.push(PropertyEntry {
                span: Span::combine(variable.span, value.span),
                kind: PropertyEntryKind::Initialized {
                    variable,
                    equals: current.span,
                    value,
                }
            });
        } else {
            entries.push(PropertyEntry {
                span: variable.span,
                kind: PropertyEntryKind::Uninitialized { variable }
            });
        }

        if state.stream.current().kind == TokenKind::Comma {
            state.stream.next();
        } else {
            break;
        }
    }

    let end = utils::skip_semicolon(state);

    Property {
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

        let current = state.stream.current();
        if current.kind == TokenKind::Equals {
            let span = current.span;
            state.stream.next();
            let value = expressions::create(state);

            entries.push(PropertyEntry{
                span: Span::combine(variable.span, value.span),
                kind: PropertyEntryKind::Initialized {
                    variable,
                    equals: span,
                    value,
                }
            });
        } else {
            entries.push(PropertyEntry {
                span: variable.span,
                kind: PropertyEntryKind::Uninitialized { variable }
            });
        }

        if state.stream.current().kind == TokenKind::Comma {
            state.stream.next();
        } else {
            break;
        }
    }

    let end = utils::skip_semicolon(state);

    VariableProperty {
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
