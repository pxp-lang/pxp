use crate::error::ParseResult;
use crate::internal::attributes;
use crate::internal::constants;
use crate::internal::functions::method;
use crate::internal::functions::Method;
use crate::internal::functions::MethodType;
use crate::internal::identifiers;
use crate::internal::modifiers;
use crate::internal::utils;
use crate::state::State;
use pxp_ast::identifiers::SimpleIdentifier;
use pxp_ast::interfaces::InterfaceBody;
use pxp_ast::interfaces::InterfaceExtends;
use pxp_ast::interfaces::InterfaceMember;
use pxp_ast::interfaces::InterfaceStatement;
use pxp_ast::Statement;
use pxp_token::TokenKind;

pub fn parse(state: &mut State) -> ParseResult<Statement> {
    let span = utils::skip(state, TokenKind::Interface)?;

    let name = identifiers::type_identifier(state)?;

    let current = state.stream.current();
    let extends = if current.kind == TokenKind::Extends {
        let span = current.span;

        state.stream.next();

        let parents =
            utils::at_least_one_comma_separated_no_trailing::<SimpleIdentifier>(state, &|state| {
                identifiers::full_type_name(state)
            })?;

        Some(InterfaceExtends {
            extends: span,
            parents,
        })
    } else {
        None
    };

    let attributes = state.get_attributes();

    let body = InterfaceBody {
        left_brace: utils::skip_left_brace(state)?,
        members: {
            let mut members = Vec::new();
            while state.stream.current().kind != TokenKind::RightBrace {
                members.push(member(state, &name)?);
            }

            members
        },
        right_brace: utils::skip_right_brace(state)?,
    };

    Ok(Statement::Interface(InterfaceStatement {
        interface: span,
        name,
        attributes,
        extends,
        body,
    }))
}

fn member(state: &mut State, interface_name: &SimpleIdentifier) -> ParseResult<InterfaceMember> {
    attributes::gather_attributes(state)?;

    let modifiers = modifiers::collect(state)?;

    if state.stream.current().kind == TokenKind::Const {
        constants::classish(state, modifiers::interface_constant_group(modifiers)?)
            .map(InterfaceMember::Constant)
    } else {
        let method = method(
            state,
            MethodType::Abstract,
            modifiers::interface_method_group(modifiers)?,
            Some(interface_name),
        )?;

        match method {
            Method::Abstract(method) => Ok(InterfaceMember::Method(method)),
            Method::AbstractConstructor(ctor) => Ok(InterfaceMember::Constructor(ctor)),
            Method::ConcreteConstructor(_) | Method::Concrete(_) => unreachable!(),
        }
    }
}
