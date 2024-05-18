use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use pxp_ast::identifiers::SimpleIdentifier;
use pxp_ast::interfaces::InterfaceBody;
use pxp_ast::interfaces::InterfaceExtends;
use pxp_ast::interfaces::InterfaceStatement;
use pxp_ast::name::Name;
use pxp_ast::StatementKind;
use pxp_ast::UseKind;
use pxp_token::TokenKind;

use super::classes::member;
use super::names;

pub fn parse(state: &mut State) -> StatementKind {
    let span = utils::skip(state, TokenKind::Interface);

    let name = names::type_name(state);

    let current = state.stream.current();
    let extends = if current.kind == TokenKind::Extends {
        let span = current.span;

        state.stream.next();

        let parents =
            utils::at_least_one_comma_separated_no_trailing::<Name>(state, &|state| {
                names::full_name(state, UseKind::Normal)
            });

        Some(InterfaceExtends {
            extends: span,
            parents,
        })
    } else {
        None
    };

    let attributes = state.get_attributes();

    let body = InterfaceBody {
        left_brace: utils::skip_left_brace(state),
        members: {
            let mut members = Vec::new();
            while state.stream.current().kind != TokenKind::RightBrace {
                members.push(member(state, true));
            }

            members
        },
        right_brace: utils::skip_right_brace(state),
    };

    StatementKind::Interface(InterfaceStatement {
        interface: span,
        name,
        attributes,
        extends,
        body,
    })
}

// fn member(state: &mut State, interface_name: &SimpleIdentifier) -> InterfaceMember {
//     attributes::gather_attributes(state);

//     let modifiers = modifiers::collect(state);

//     if state.stream.current().kind == TokenKind::Const {
//         constants::classish(state, modifiers::interface_constant_group(modifiers))
//             .map(InterfaceMember::Constant)
//     } else {
//         let method = method(
//             state,
//             MethodType::Abstract,
//             modifiers::interface_method_group(modifiers),
//             Some(interface_name),
//         );

//         match method {
//             Method::Abstract(method) => Ok(InterfaceMember::Method(method)),
//             Method::AbstractConstructor(ctor) => Ok(InterfaceMember::Constructor(ctor)),
//             Method::ConcreteConstructor(_) | Method::Concrete(_) => unreachable!(),
//         }
//     }
// }
