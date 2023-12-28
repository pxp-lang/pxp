use crate::expressions;
use crate::internal::data_type;
use crate::internal::utils;
use crate::internal::variables;
use crate::state::State;
use pxp_ast::modifiers::PropertyModifierGroup;
use pxp_ast::properties::Property;
use pxp_ast::properties::PropertyEntry;
use pxp_ast::properties::VariableProperty;
use pxp_diagnostics::DiagnosticKind;
use pxp_diagnostics::Severity;
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
                    DiagnosticKind::StaticPropertyCannotBeReadonly,
                    Severity::Error,
                    state.stream.current().span,
                );
            }

            match &ty {
                Some(ty) => {
                    if ty.includes_callable() || ty.is_bottom() {
                        state.diagnostic(
                            DiagnosticKind::ForbiddenTypeUsedInProperty,
                            Severity::Error,
                            ty.get_span(),
                        );
                    }
                }
                None => {
                    if let Some(modifier) = modifiers.get_readonly() {
                        state.diagnostic(
                            DiagnosticKind::ReadonlyPropertyMustHaveType,
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
                    DiagnosticKind::ReadonlyPropertyCannotHaveDefaultValue,
                    Severity::Error,
                    modifier.span(),
                );
            }

            state.stream.next();
            let value = expressions::create(state);

            entries.push(PropertyEntry::Initialized {
                variable,
                equals: current.span,
                value,
            });
        } else {
            entries.push(PropertyEntry::Uninitialized { variable });
        }

        if state.stream.current().kind == TokenKind::Comma {
            state.stream.next();
        } else {
            break;
        }
    }

    let end = utils::skip_semicolon(state);

    Property {
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

    let mut entries = vec![];
    let mut type_checked = false;
    loop {
        let variable = variables::simple_variable(state);

        if !type_checked {
            type_checked = true;

            if let Some(ty) = &ty {
                if ty.includes_callable() || ty.is_bottom() {
                    state.diagnostic(
                        DiagnosticKind::ForbiddenTypeUsedInProperty,
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

            entries.push(PropertyEntry::Initialized {
                variable,
                equals: span,
                value,
            });
        } else {
            entries.push(PropertyEntry::Uninitialized { variable });
        }

        if state.stream.current().kind == TokenKind::Comma {
            state.stream.next();
        } else {
            break;
        }
    }

    let end = utils::skip_semicolon(state);

    VariableProperty {
        r#type: ty,
        attributes: state.get_attributes(),
        entries,
        end,
    }
}
