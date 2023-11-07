use crate::lexer::token::Span;
use crate::lexer::token::TokenKind;
use crate::parser::ast::classes::AnonymousClassBody;
use crate::parser::ast::classes::AnonymousClassExpression;
use crate::parser::ast::classes::AnonymousClassMember;
use crate::parser::ast::classes::ClassBody;
use crate::parser::ast::classes::ClassExtends;
use crate::parser::ast::classes::ClassImplements;
use crate::parser::ast::classes::ClassMember;
use crate::parser::ast::classes::ClassStatement;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::ast::Statement;
use crate::parser::ast::{Expression, NewExpression};
use crate::parser::error;
use crate::parser::error::ParseResult;
use crate::parser::internal::attributes;
use crate::parser::internal::constants::classish;
use crate::parser::internal::functions::method;
use crate::parser::internal::functions::Method;
use crate::parser::internal::functions::MethodType;
use crate::parser::internal::identifiers;
use crate::parser::internal::modifiers;
use crate::parser::internal::parameters;
use crate::parser::internal::properties;
use crate::parser::internal::traits;
use crate::parser::internal::utils;
use crate::parser::state::State;

pub fn parse(state: &mut State) -> ParseResult<Statement> {
    let attributes = state.get_attributes();

    let modifiers = modifiers::class_group(modifiers::collect(state)?)?;
    let class = utils::skip(state, TokenKind::Class)?;
    let name = identifiers::type_identifier(state)?;
    let current = state.stream.current();
    let extends = if current.kind == TokenKind::Extends {
        let span = current.span;

        state.stream.next();
        let parent = identifiers::full_type_name(state)?;

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
            utils::at_least_one_comma_separated_no_trailing::<SimpleIdentifier>(state, &|state| {
                identifiers::full_type_name(state)
            })?;

        Some(ClassImplements {
            implements: span,
            interfaces,
        })
    } else {
        None
    };

    let has_abstract = modifiers.has_abstract();
    let body = ClassBody {
        left_brace: utils::skip_left_brace(state)?,
        members: {
            let mut members = Vec::new();
            while state.stream.current().kind != TokenKind::RightBrace {
                members.push(member(state, has_abstract, &name)?);
            }

            members
        },
        right_brace: utils::skip_right_brace(state)?,
    };

    Ok(Statement::Class(ClassStatement {
        class,
        name,
        modifiers,
        extends,
        implements,
        attributes,
        body,
    }))
}

pub fn parse_anonymous(state: &mut State, span: Option<Span>) -> ParseResult<Expression> {
    let new = match span {
        Some(span) => span,
        None => utils::skip(state, TokenKind::New)?,
    };

    attributes::gather_attributes(state)?;

    let attributes = state.get_attributes();

    let class = utils::skip(state, TokenKind::Class)?;

    let arguments = if state.stream.current().kind == TokenKind::LeftParen {
        Some(parameters::argument_list(state)?)
    } else {
        None
    };

    let current = state.stream.current();
    let extends = if current.kind == TokenKind::Extends {
        state.stream.next();

        let extends = current.span;
        let parent = identifiers::full_name(state)?;

        Some(ClassExtends { extends, parent })
    } else {
        None
    };

    let current = state.stream.current();
    let implements = if current.kind == TokenKind::Implements {
        state.stream.next();

        let implements = current.span;
        let interfaces =
            utils::at_least_one_comma_separated_no_trailing::<SimpleIdentifier>(state, &|state| {
                identifiers::full_name(state)
            })?;

        Some(ClassImplements {
            implements,
            interfaces,
        })
    } else {
        None
    };

    let body = AnonymousClassBody {
        left_brace: utils::skip_left_brace(state)?,
        members: {
            let mut members = Vec::new();
            while state.stream.current().kind != TokenKind::RightBrace {
                members.push(anonymous_member(state)?);
            }
            members
        },
        right_brace: utils::skip_right_brace(state)?,
    };

    Ok(Expression::New(NewExpression {
        target: Box::new(Expression::AnonymousClass(AnonymousClassExpression {
            class,
            extends,
            implements,
            attributes,
            body,
        })),
        new,
        arguments,
    }))
}

fn member(
    state: &mut State,
    has_abstract: bool,
    name: &SimpleIdentifier,
) -> ParseResult<ClassMember> {
    let has_attributes = attributes::gather_attributes(state)?;

    if !has_attributes && state.stream.current().kind == TokenKind::Use {
        return traits::usage(state).map(ClassMember::TraitUsage);
    }

    if state.stream.current().kind == TokenKind::Var {
        return properties::parse_var(state, Some(name)).map(ClassMember::VariableProperty);
    }

    let modifiers = modifiers::collect(state)?;

    if state.stream.current().kind == TokenKind::Const {
        return classish(state, modifiers::constant_group(modifiers)?).map(ClassMember::Constant);
    }

    if state.stream.current().kind == TokenKind::Function {
        let method = method(
            state,
            MethodType::DependingOnModifiers,
            modifiers::method_group(modifiers)?,
            Some(name),
        )?;

        return match method {
            Method::Abstract(method) => {
                if has_abstract {
                    Ok(ClassMember::AbstractMethod(method))
                } else {
                    Err(error::abstract_method_on_a_non_abstract_class(
                        state,
                        name,
                        &method.name,
                        method.modifiers.get_abstract().unwrap().span(),
                        method.semicolon,
                    ))
                }
            }
            Method::Concrete(method) => Ok(ClassMember::ConcreteMethod(method)),
            Method::AbstractConstructor(ctor) => {
                if has_abstract {
                    Ok(ClassMember::AbstractConstructor(ctor))
                } else {
                    Err(error::abstract_method_on_a_non_abstract_class(
                        state,
                        name,
                        &ctor.name,
                        ctor.modifiers.get_abstract().unwrap().span(),
                        ctor.semicolon,
                    ))
                }
            }
            Method::ConcreteConstructor(ctor) => Ok(ClassMember::ConcreteConstructor(ctor)),
        };
    }

    // e.g: public static
    let modifiers = modifiers::property_group(modifiers)?;

    properties::parse(state, Some(name), modifiers).map(ClassMember::Property)
}

fn anonymous_member(state: &mut State) -> ParseResult<AnonymousClassMember> {
    let has_attributes = attributes::gather_attributes(state)?;

    if !has_attributes && state.stream.current().kind == TokenKind::Use {
        return traits::usage(state).map(AnonymousClassMember::TraitUsage);
    }

    if state.stream.current().kind == TokenKind::Var {
        return properties::parse_var(state, None).map(AnonymousClassMember::VariableProperty);
    }

    let modifiers = modifiers::collect(state)?;

    if state.stream.current().kind == TokenKind::Const {
        return classish(state, modifiers::constant_group(modifiers)?)
            .map(AnonymousClassMember::Constant);
    }

    if state.stream.current().kind == TokenKind::Function {
        let method = method(
            state,
            MethodType::Concrete,
            modifiers::method_group(modifiers)?,
            None,
        )?;

        match method {
            Method::Concrete(method) => {
                return Ok(AnonymousClassMember::ConcreteMethod(method));
            }
            Method::ConcreteConstructor(ctor) => {
                return Ok(AnonymousClassMember::ConcreteConstructor(ctor));
            }
            Method::Abstract(_) | Method::AbstractConstructor(_) => unreachable!(),
        }
    }

    // e.g: public static
    let modifiers = modifiers::property_group(modifiers)?;

    properties::parse(state, None, modifiers).map(AnonymousClassMember::Property)
}
