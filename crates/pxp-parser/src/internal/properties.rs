use crate::expressions;
use crate::internal::data_type;
use crate::internal::utils;
use crate::internal::variables;
use crate::state::State;
use pxp_ast::identifiers::SimpleIdentifier;
use pxp_ast::modifiers::PropertyModifierGroup;
use pxp_ast::properties::Property;
use pxp_ast::properties::PropertyEntry;
use pxp_ast::properties::VariableProperty;
use pxp_token::TokenKind;

pub fn parse(
    state: &mut State,
    class_name: Option<&SimpleIdentifier>,
    modifiers: PropertyModifierGroup,
) -> Property {
    let ty = data_type::optional_data_type(state);

    let mut entries = vec![];
    let mut type_checked = false;
    loop {
        let variable = variables::simple_variable(state);

        if !type_checked {
            type_checked = true;
            if modifiers.has_readonly() && modifiers.has_static() {
                todo!("tolerant mode")
                // let error = error::static_property_cannot_be_readonly(
                //     state,
                //     class_name,
                //     &variable,
                //     modifiers.get_static().unwrap().span(),
                //     modifiers.get_readonly().unwrap().span(),
                // );

                // state.record(error);
            }

            match &ty {
                Some(ty) => {
                    if ty.includes_callable() || ty.is_bottom() {
                        todo!("tolerant mode")
                        // let error = error::forbidden_type_used_in_property(
                        //     state,
                        //     class_name,
                        //     &variable,
                        //     ty.clone(),
                        // );

                        // state.record(error);
                    }
                }
                None => {
                    if let Some(modifier) = modifiers.get_readonly() {
                        todo!("tolerant mode")
                        // let error = error::missing_type_for_readonly_property(
                        //     state,
                        //     class_name,
                        //     &variable,
                        //     modifier.span(),
                        // );

                        // state.record(error);
                    }
                }
            }
        }

        let current = state.stream.current();
        if current.kind == TokenKind::Equals {
            if let Some(modifier) = modifiers.get_readonly() {
                todo!("tolerant mode")
                // let error = error::readonly_property_has_default_value(
                //     state,
                //     class_name,
                //     &variable,
                //     modifier.span(),
                //     current.span,
                // );

                // state.record(error);
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

pub fn parse_var(state: &mut State, class_name: Option<&SimpleIdentifier>) -> VariableProperty {
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
                    todo!("tolerant mode")
                    // let error = error::forbidden_type_used_in_property(
                    //     state,
                    //     class_name,
                    //     &variable,
                    //     ty.clone(),
                    // );

                    // state.record(error);
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
