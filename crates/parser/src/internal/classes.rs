use crate::internal::attributes;
use crate::internal::constants::parse_classish_constant;
use crate::internal::functions::parse_method;
use crate::internal::functions::Method;
use crate::internal::modifiers;
use crate::internal::parameters;
use crate::internal::properties;
use crate::internal::traits;
use crate::internal::utils;
use crate::state::State;
use crate::ParserDiagnostic;
use pxp_ast::Expression;
use pxp_ast::StatementKind;
use pxp_ast::UseKind;
use pxp_ast::*;
use pxp_ast::{ExpressionKind, NewExpression};

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

use super::names;

pub fn parse_class(state: &mut State) -> StatementKind {
    let attributes = state.get_attributes();

    let modifiers = modifiers::collect_modifiers(state);
    let modifiers = modifiers::parse_class_group(state, modifiers);
    let class = utils::skip(state, TokenKind::Class);
    let name = names::parse_type_name(state);
    let current = state.current();
    let extends = if current.kind == TokenKind::Extends {
        let span = current.span;

        state.next();
        let parent = names::parse_full_name(state, UseKind::Normal);

        Some(ClassExtends {
            id: state.id(),
            span: Span::combine(span, parent.span),
            extends: span,
            parent,
        })
    } else {
        None
    };

    let current = state.current();
    let implements = if current.kind == TokenKind::Implements {
        let span = current.span;

        state.next();

        let interfaces = utils::at_least_one_comma_separated_no_trailing::<Name>(state, &|state| {
            names::parse_full_name(state, UseKind::Normal)
        });

        Some(ClassImplements {
            id: state.id(),
            span: Span::combine(span, interfaces.span()),
            implements: span,
            interfaces,
        })
    } else {
        None
    };

    let has_abstract = modifiers.has_abstract();
    let left_brace = utils::skip_left_brace(state);
    let members = {
        let mut members = Vec::new();
        while state.current().kind != TokenKind::RightBrace {
            if state.is_eof() {
                break;
            }

            members.push(parse_classish_member(state, has_abstract));
        }

        members
    };
    let right_brace = utils::skip_right_brace(state);

    let body = ClassBody {
        id: state.id(),
        span: Span::combine(left_brace, right_brace),
        left_brace,
        members,
        right_brace,
    };

    let span = if !modifiers.is_empty() {
        Span::combine(modifiers.span(), body.span)
    } else {
        Span::combine(class, body.span)
    };

    StatementKind::Class(ClassStatement {
        id: state.id(),
        span,
        class,
        name,
        modifiers,
        extends,
        implements,
        attributes,
        body,
    })
}

pub fn parse_anonymous_class(state: &mut State, span: Option<Span>) -> Expression {
    let new = match span {
        Some(span) => span,
        None => utils::skip(state, TokenKind::New),
    };

    let start_span = new;

    attributes::gather_attributes(state);

    let attributes = state.get_attributes();
    let class = utils::skip(state, TokenKind::Class);
    let class_span = class;

    let arguments = if state.current().kind == TokenKind::LeftParen {
        Some(parameters::parse_argument_list(state))
    } else {
        None
    };

    let current = state.current();
    let extends = if current.kind == TokenKind::Extends {
        state.next();

        let extends = current.span;
        let parent = names::parse_full_name(state, UseKind::Normal);

        Some(ClassExtends {
            id: state.id(),
            span: Span::combine(extends, parent.span),
            extends,
            parent,
        })
    } else {
        None
    };

    let current = state.current();
    let implements = if current.kind == TokenKind::Implements {
        state.next();

        let implements = current.span;
        let interfaces = utils::at_least_one_comma_separated_no_trailing::<Name>(state, &|state| {
            names::parse_full_name(state, UseKind::Normal)
        });

        Some(ClassImplements {
            id: state.id(),
            span: Span::combine(implements, interfaces.span()),
            implements,
            interfaces,
        })
    } else {
        None
    };

    let left_brace = utils::skip_left_brace(state);
    let members = {
        let mut members = Vec::new();
        while state.current().kind != TokenKind::RightBrace {
            members.push(parse_classish_member(state, false));
        }
        members
    };
    let right_brace = utils::skip_right_brace(state);
    let span = Span::combine(left_brace, right_brace);

    let body = AnonymousClassBody {
        id: state.id(),
        span,
        left_brace,
        members,
        right_brace,
    };

    let end_span = body.right_brace;

    let anonymous_class = Expression::new(
        state.id(),
        ExpressionKind::AnonymousClass(AnonymousClassExpression {
            id: state.id(),
            span: Span::combine(class, body.span),
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
        state.id(),
        ExpressionKind::New(NewExpression {
            id: state.id(),
            span: Span::combine(new, arguments.span()),
            target: Box::new(anonymous_class),
            new,
            arguments,
        }),
        Span::new(start_span.start, state.previous().span.end),
        CommentGroup::default(),
    )
}

pub fn parse_classish_member(state: &mut State, has_abstract: bool) -> ClassishMember {
    let has_attributes = attributes::gather_attributes(state);

    if !has_attributes && state.current().kind == TokenKind::Use {
        return ClassishMember::TraitUsage(traits::parse_trait_usage(state));
    }

    if state.current().kind == TokenKind::Var {
        return ClassishMember::VariableProperty(properties::parse_var_property(state));
    }

    let modifiers = modifiers::collect_modifiers(state);

    if modifiers.is_empty()
        && !matches!(state.current().kind, TokenKind::Const | TokenKind::Function)
    {
        let current = state.current();

        state.diagnostic(
            ParserDiagnostic::UnexpectedToken {
                token: current.clone(),
            },
            Severity::Error,
            current.span,
        );

        state.next();

        return ClassishMember::Missing(MissingClassishMember {
            id: state.id(),
            span: current.span,
        });
    }

    if state.current().kind == TokenKind::Const {
        let modifiers = modifiers::parse_constant_group(state, modifiers);
        return ClassishMember::Constant(parse_classish_constant(state, modifiers));
    }

    if state.current().kind == TokenKind::Function {
        let modifiers = modifiers::parse_method_group(state, modifiers);
        let method = parse_method(state, modifiers);

        return match method {
            Method::Abstract(method) => {
                if !has_abstract && method.modifiers.has_abstract() {
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
                        ctor.span(),
                    );
                }

                ClassishMember::AbstractConstructor(ctor)
            }
            Method::ConcreteConstructor(ctor) => ClassishMember::ConcreteConstructor(ctor),
        };
    }

    // e.g: public static
    let modifiers = modifiers::parse_property_group(state, modifiers);

    ClassishMember::Property(properties::parse_property(state, modifiers))
}
