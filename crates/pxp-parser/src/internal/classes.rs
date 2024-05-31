use crate::internal::attributes;
use crate::internal::constants::classish;
use crate::internal::functions::method;
use crate::internal::functions::Method;
use crate::internal::modifiers;
use crate::internal::parameters;
use crate::internal::properties;
use crate::internal::traits;
use crate::internal::utils;
use crate::state::State;
use crate::ParserDiagnostic;
use pxp_ast::*;
use pxp_ast::Expression;
use pxp_ast::StatementKind;
use pxp_ast::UseKind;
use pxp_ast::{ExpressionKind, NewExpression};

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_syntax::comments::CommentGroup;
use pxp_token::TokenKind;

use super::names;

pub fn parse(state: &mut State) -> StatementKind {
    let attributes = state.get_attributes();

    let modifiers = modifiers::collect(state);
    let modifiers = modifiers::class_group(state, modifiers);
    let class = utils::skip(state, TokenKind::Class);
    let name = names::type_name(state);
    let current = state.stream.current();
    let extends = if current.kind == TokenKind::Extends {
        let span = current.span;

        state.stream.next();
        let parent = names::full_name(state, UseKind::Normal);

        Some(ClassExtends {
            extends: span,
            parent,
        })
    } else {
        None
    };

    let current = state.stream.current();
    let implements = if current.kind == TokenKind::Implements {
        let span = current.span;

        state.stream.next();

        let interfaces =
            utils::at_least_one_comma_separated_no_trailing::<Name>(state, &|state| {
                names::full_name(state, UseKind::Normal)
            });

        Some(ClassImplements {
            implements: span,
            interfaces,
        })
    } else {
        None
    };

    let has_abstract = modifiers.has_abstract();
    let body = ClassBody {
        left_brace: utils::skip_left_brace(state),
        members: {
            let mut members = Vec::new();
            while state.stream.current().kind != TokenKind::RightBrace {
                if state.stream.is_eof() {
                    break;
                }

                members.push(member(state, has_abstract));
            }

            members
        },
        right_brace: utils::skip_right_brace(state),
    };

    StatementKind::Class(ClassStatement {
        class,
        name,
        modifiers,
        extends,
        implements,
        attributes,
        body,
    })
}

pub fn parse_anonymous(state: &mut State, span: Option<Span>) -> Expression {
    let new = match span {
        Some(span) => span,
        None => utils::skip(state, TokenKind::New),
    };

    let start_span = new;

    attributes::gather_attributes(state);

    let attributes = state.get_attributes();
    let class = utils::skip(state, TokenKind::Class);
    let class_span = class;

    let arguments = if state.stream.current().kind == TokenKind::LeftParen {
        Some(parameters::argument_list(state))
    } else {
        None
    };

    let current = state.stream.current();
    let extends = if current.kind == TokenKind::Extends {
        state.stream.next();

        let extends = current.span;
        let parent = names::full_name(state, UseKind::Normal);

        Some(ClassExtends { extends, parent })
    } else {
        None
    };

    let current = state.stream.current();
    let implements = if current.kind == TokenKind::Implements {
        state.stream.next();

        let implements = current.span;
        let interfaces =
            utils::at_least_one_comma_separated_no_trailing::<Name>(state, &|state| {
                names::full_name(state, UseKind::Normal)
            });

        Some(ClassImplements {
            implements,
            interfaces,
        })
    } else {
        None
    };

    let body = AnonymousClassBody {
        left_brace: utils::skip_left_brace(state),
        members: {
            let mut members = Vec::new();
            while state.stream.current().kind != TokenKind::RightBrace {
                members.push(member(state, false));
            }
            members
        },
        right_brace: utils::skip_right_brace(state),
    };

    let end_span = body.right_brace;

    let anonymous_class = Expression::new(
        ExpressionKind::AnonymousClass(AnonymousClassExpression {
            class,
            extends,
            implements,
            attributes,
            body,
        }),
        Span::new(class_span.start, end_span.end),
        CommentGroup::default(),
    );

    Expression::new(
        ExpressionKind::New(NewExpression {
            target: Box::new(anonymous_class),
            new,
            arguments,
        }),
        Span::new(start_span.start, state.stream.previous().span.end),
        CommentGroup::default(),
    )
}

pub fn member(state: &mut State, has_abstract: bool) -> ClassishMember {
    let has_attributes = attributes::gather_attributes(state);

    if !has_attributes && state.stream.current().kind == TokenKind::Use {
        return ClassishMember::TraitUsage(traits::usage(state));
    }

    if state.stream.current().kind == TokenKind::Var {
        return ClassishMember::VariableProperty(properties::parse_var(state));
    }

    let modifiers = modifiers::collect(state);

    if state.stream.current().kind == TokenKind::Const {
        let modifiers = modifiers::constant_group(state, modifiers);
        return ClassishMember::Constant(classish(state, modifiers));
    }

    if state.stream.current().kind == TokenKind::Function {
        let modifiers = modifiers::method_group(state, modifiers);
        let method = method(state, modifiers);

        return match method {
            Method::Abstract(method) => {
                if !has_abstract {
                    state.diagnostic(
                        ParserDiagnostic::AbstractMethodInNonAbstractClass,
                        Severity::Error,
                        method.modifiers.get_abstract().unwrap().span(),
                    );
                }

                ClassishMember::AbstractMethod(method)
            }
            Method::Concrete(method) => ClassishMember::ConcreteMethod(method),
            Method::AbstractConstructor(ctor) => {
                if !has_abstract {
                    state.diagnostic(
                        ParserDiagnostic::AbstractMethodInNonAbstractClass,
                        Severity::Error,
                        ctor.modifiers.get_abstract().unwrap().span(),
                    );
                }

                ClassishMember::AbstractConstructor(ctor)
            }
            Method::ConcreteConstructor(ctor) => ClassishMember::ConcreteConstructor(ctor),
        };
    }

    // e.g: public static
    let modifiers = modifiers::property_group(state, modifiers);
    
    ClassishMember::Property(properties::parse(state, modifiers))
}
