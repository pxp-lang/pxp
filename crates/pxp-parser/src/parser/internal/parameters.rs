use crate::lexer::token::TokenKind;
use crate::parser::ast::arguments::{Argument, SingleArgument};
use crate::parser::ast::arguments::{ArgumentList, NamedArgument, PositionalArgument};
use crate::parser::ast::functions::ConstructorParameter;
use crate::parser::ast::functions::ConstructorParameterList;
use crate::parser::ast::functions::FunctionParameter;
use crate::parser::ast::functions::FunctionParameterList;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::error;
use crate::parser::error::ParseError;
use crate::parser::error::ParseResult;
use crate::parser::expressions;
use crate::parser::internal::attributes;
use crate::parser::internal::data_type;
use crate::parser::internal::identifiers;
use crate::parser::internal::modifiers;
use crate::parser::internal::utils;
use crate::parser::internal::variables;
use crate::parser::state::State;

pub fn function_parameter_list(state: &mut State) -> Result<FunctionParameterList, ParseError> {
    let comments = state.stream.comments();
    let left_parenthesis = utils::skip_left_parenthesis(state)?;
    let parameters = utils::comma_separated(
        state,
        &|state| {
            attributes::gather_attributes(state)?;

            let ty = data_type::optional_data_type(state)?;

            let mut current = state.stream.current();
            let ampersand = if current.kind == TokenKind::Ampersand {
                state.stream.next();
                current = state.stream.current();
                Some(current.span)
            } else {
                None
            };

            let ellipsis = if current.kind == TokenKind::Ellipsis {
                state.stream.next();

                Some(current.span)
            } else {
                None
            };

            // 2. Then expect a variable.
            let var = variables::simple_variable(state)?;

            let mut default = None;
            if state.stream.current().kind == TokenKind::Equals {
                state.stream.next();
                default = Some(expressions::create(state)?);
            }

            Ok(FunctionParameter {
                comments: state.stream.comments(),
                name: var,
                attributes: state.get_attributes(),
                data_type: ty,
                ellipsis,
                default,
                ampersand,
            })
        },
        TokenKind::RightParen,
    )?;

    let right_parenthesis = utils::skip_right_parenthesis(state)?;

    Ok(FunctionParameterList {
        comments,
        left_parenthesis,
        parameters,
        right_parenthesis,
    })
}

pub fn constructor_parameter_list(
    state: &mut State,
    class: Option<&SimpleIdentifier>,
) -> Result<ConstructorParameterList, ParseError> {
    let comments = state.stream.comments();

    let left_parenthesis = utils::skip_left_parenthesis(state)?;
    let parameters = utils::comma_separated::<ConstructorParameter>(
        state,
        &|state| {
            attributes::gather_attributes(state)?;

            let modifiers = modifiers::promoted_property_group(modifiers::collect(state)?)?;

            let ty = data_type::optional_data_type(state)?;

            let mut current = state.stream.current();
            let ampersand = if matches!(current.kind, TokenKind::Ampersand) {
                state.stream.next();

                current = state.stream.current();

                Some(current.span)
            } else {
                None
            };

            let (ellipsis, var) = if matches!(current.kind, TokenKind::Ellipsis) {
                state.stream.next();
                let var = variables::simple_variable(state)?;
                if !modifiers.is_empty() {
                    return Err(error::variadic_promoted_property(
                        state,
                        class,
                        &var,
                        current.span,
                        modifiers.modifiers.first().unwrap(),
                    ));
                }

                (Some(current.span), var)
            } else {
                (None, variables::simple_variable(state)?)
            };

            // 2. Then expect a variable.

            if !modifiers.is_empty() {
                match &ty {
                    Some(ty) => {
                        if ty.includes_callable() || ty.is_bottom() {
                            return Err(error::forbidden_type_used_in_property(
                                state,
                                class,
                                &var,
                                ty.clone(),
                            ));
                        }
                    }
                    None => {
                        if let Some(modifier) = modifiers.get_readonly() {
                            return Err(error::missing_type_for_readonly_property(
                                state,
                                class,
                                &var,
                                modifier.span(),
                            ));
                        }
                    }
                }
            }

            let mut default = None;
            if state.stream.current().kind == TokenKind::Equals {
                state.stream.next();
                default = Some(expressions::create(state)?);
            }

            Ok(ConstructorParameter {
                comments: state.stream.comments(),
                name: var,
                attributes: state.get_attributes(),
                data_type: ty,
                ellipsis,
                default,
                modifiers,
                ampersand,
            })
        },
        TokenKind::RightParen,
    )?;

    let right_parenthesis = utils::skip_right_parenthesis(state)?;

    Ok(ConstructorParameterList {
        comments,
        left_parenthesis,
        parameters,
        right_parenthesis,
    })
}

pub fn argument_list(state: &mut State) -> ParseResult<ArgumentList> {
    let comments = state.stream.comments();
    let start = utils::skip_left_parenthesis(state)?;

    let mut arguments = Vec::new();
    let mut has_used_named_arguments = false;

    while !state.stream.is_eof() && state.stream.current().kind != TokenKind::RightParen {
        let span = state.stream.current().span;
        let (named, argument) = argument(state)?;
        if named {
            has_used_named_arguments = true;
        } else if has_used_named_arguments {
            return Err(error::cannot_use_positional_argument_after_named_argument(
                span,
                state.stream.current().span,
            ));
        }

        arguments.push(argument);

        if state.stream.current().kind == TokenKind::Comma {
            state.stream.next();
        } else {
            break;
        }
    }

    let end = utils::skip_right_parenthesis(state)?;

    Ok(ArgumentList {
        comments,
        left_parenthesis: start,
        right_parenthesis: end,
        arguments,
    })
}

pub fn single_argument(
    state: &mut State,
    required: bool,
    only_positional: bool,
) -> Option<ParseResult<SingleArgument>> {
    let comments = state.stream.comments();
    let start = utils::skip_left_parenthesis(state).ok()?;

    let mut first_argument = None;

    while !state.stream.is_eof() && state.stream.current().kind != TokenKind::RightParen {
        let span = state.stream.current().span;
        let (named, argument) = argument(state).ok()?;
        if only_positional && named {
            return Some(Err(error::only_positional_arguments_are_accepted(
                span,
                state.stream.current().span,
            )));
        }

        if first_argument.is_some() {
            return Some(Err(error::only_one_argument_is_accepted(
                span,
                state.stream.current().span,
            )));
        }

        first_argument = Some(argument);

        if state.stream.current().kind == TokenKind::Comma {
            state.stream.next();
        } else {
            break;
        }
    }

    if required && first_argument.is_none() {
        return Some(Err(error::argument_is_required(
            state.stream.current().span,
            state.stream.current().span,
        )));
    }

    let end = utils::skip_right_parenthesis(state).ok()?;

    first_argument.as_ref()?;

    Some(Ok(SingleArgument {
        comments,
        left_parenthesis: start,
        right_parenthesis: end,
        argument: first_argument.unwrap(),
    }))
}

fn argument(state: &mut State) -> ParseResult<(bool, Argument)> {
    if identifiers::is_identifier_maybe_reserved(&state.stream.current().kind)
        && state.stream.peek().kind == TokenKind::Colon
    {
        let name = identifiers::identifier_maybe_reserved(state)?;
        let colon = utils::skip(state, TokenKind::Colon)?;
        let ellipsis = if state.stream.current().kind == TokenKind::Ellipsis {
            Some(utils::skip(state, TokenKind::Ellipsis)?)
        } else {
            None
        };
        let value = expressions::create(state)?;

        return Ok((
            true,
            Argument::Named(NamedArgument {
                comments: state.stream.comments(),
                name,
                colon,
                ellipsis,
                value,
            }),
        ));
    }

    let ellipsis = if state.stream.current().kind == TokenKind::Ellipsis {
        Some(utils::skip(state, TokenKind::Ellipsis)?)
    } else {
        None
    };

    let value = expressions::create(state)?;

    Ok((
        false,
        Argument::Positional(PositionalArgument {
            comments: state.stream.comments(),
            ellipsis,
            value,
        }),
    ))
}
