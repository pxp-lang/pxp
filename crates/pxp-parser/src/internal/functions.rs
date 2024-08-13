use crate::expressions;
use crate::internal::blocks;
use crate::internal::data_type;
use crate::internal::identifiers;
use crate::internal::parameters;
use crate::internal::utils;
use crate::internal::variables;
use crate::state::State;
use pxp_ast::Expression;
use pxp_ast::ExpressionKind;
use pxp_ast::StatementKind;
use pxp_ast::*;
use pxp_span::Span;
use pxp_syntax::comments::CommentGroup;
use pxp_token::TokenKind;

use super::names;

pub enum Method {
    Abstract(AbstractMethod),
    Concrete(ConcreteMethod),
    AbstractConstructor(AbstractConstructor),
    ConcreteConstructor(ConcreteConstructor),
}

pub fn anonymous_function(state: &mut State) -> Expression {
    let comments = state.stream.comments();
    let start_span = state.stream.current().span;
    let attributes = state.get_attributes();
    let current = state.stream.current();
    let r#static = if current.kind == TokenKind::Static {
        state.stream.next();

        Some(current.span)
    } else {
        None
    };

    let function = utils::skip(state, TokenKind::Function);

    let current = state.stream.current();
    let ampersand = if current.kind == TokenKind::Ampersand {
        state.stream.next();

        Some(current.span)
    } else {
        None
    };

    let parameters = parameters::function_parameter_list(state);

    let current = state.stream.current();
    let uses = if current.kind == TokenKind::Use {
        state.stream.next();

        let left_parenthesis = utils::skip_left_parenthesis(state);
        let variables = utils::comma_separated::<ClosureUseVariable>(
            state,
            &|state| {
                let use_comments = state.stream.comments();
                let current = state.stream.current();
                let use_ampersand = if current.kind == TokenKind::Ampersand {
                    state.stream.next();

                    Some(current.span)
                } else {
                    None
                };

                let var = variables::simple_variable(state);

                ClosureUseVariable {
                    id: state.id(),
                    span: var.span,
                    comments: use_comments,
                    variable: var,
                    ampersand: use_ampersand,
                }
            },
            TokenKind::RightParen,
        );

        let right_parenthesis = utils::skip_right_parenthesis(state);

        Some(ClosureUse {
            id: state.id(),
            span: Span::combine(current.span, right_parenthesis),
            comments: state.stream.comments(),
            r#use: current.span,
            left_parenthesis,
            variables,
            right_parenthesis,
        })
    } else {
        None
    };

    let return_type = if state.stream.current().kind == TokenKind::Colon {
        let colon = utils::skip_colon(state);
        let data_type = data_type::data_type(state);

        Some(ReturnType {
            id: state.id(),
            span: Span::combine(colon, data_type.span),
            colon,
            data_type,
        })
    } else {
        None
    };

    let body_comments = state.stream.comments();
    let left_brace = utils::skip_left_brace(state);
    let statements = blocks::multiple_statements_until(state, &TokenKind::RightBrace);
    let right_brace = utils::skip_right_brace(state);

    let body = FunctionBody {
        id: state.id(),
        span: Span::combine(left_brace, right_brace),
        comments: body_comments,
        left_brace,
        statements,
        right_brace,
    };

    let end_span = body.right_brace;

    Expression::new(
        state.id(),
        ExpressionKind::Closure(ClosureExpression {
            id: state.id(),
            span: Span::combine(function, body.span),
            comments,
            attributes,
            r#static,
            function,
            ampersand,
            parameters,
            uses,
            return_type,
            body,
        }),
        Span::new(start_span.start, end_span.end),
        CommentGroup::default(),
    )
}

pub fn arrow_function(state: &mut State) -> Expression {
    let comments = state.stream.comments();
    let start_span = state.stream.current().span;
    let current = state.stream.current();
    let r#static = if current.kind == TokenKind::Static {
        state.stream.next();

        Some(current.span)
    } else {
        None
    };

    let r#fn = utils::skip(state, TokenKind::Fn);

    let current = state.stream.current();
    let ampersand = if state.stream.current().kind == TokenKind::Ampersand {
        state.stream.next();

        Some(current.span)
    } else {
        None
    };

    let attributes = state.get_attributes();
    let parameters = parameters::function_parameter_list(state);
    let return_type = if state.stream.current().kind == TokenKind::Colon {
        let colon = utils::skip_colon(state);
        let data_type = data_type::data_type(state);

        Some(ReturnType {
            id: state.id(),
            span: Span::combine(colon, data_type.span),
            colon,
            data_type,
        })
    } else {
        None
    };

    let double_arrow = utils::skip(state, TokenKind::DoubleArrow);

    let body = Box::new(expressions::create(state));
    let end_span = body.span;

    Expression::new(
        state.id(),
        ExpressionKind::ArrowFunction(ArrowFunctionExpression {
            id: state.id(),
            span: Span::combine(r#fn, end_span),
            comments,
            attributes,
            r#static,
            r#fn,
            ampersand,
            parameters,
            return_type,
            double_arrow,
            body,
        }),
        Span::new(start_span.start, end_span.end),
        CommentGroup::default(),
    )
}

pub fn function(state: &mut State) -> StatementKind {
    let comments = state.stream.comments();

    let function = utils::skip(state, TokenKind::Function);

    let current = state.stream.current();
    let ampersand = if current.kind == TokenKind::Ampersand {
        state.stream.next();

        Some(current.span)
    } else {
        None
    };

    let name = names::type_name_maybe_soft_reserved(state);

    // get attributes before processing parameters, otherwise
    // parameters will steal attributes of this function.
    let attributes = state.get_attributes();

    let parameters = parameters::function_parameter_list(state);
    let return_type = if state.stream.current().kind == TokenKind::Colon {
        let colon = utils::skip_colon(state);
        let data_type = data_type::data_type(state);

        Some(ReturnType {
            id: state.id(),
            span: Span::combine(colon, data_type.span),
            colon,
            data_type,
        })
    } else {
        None
    };

    let body_comments = state.stream.comments();
    let left_brace = utils::skip_left_brace(state);
    let statements = blocks::multiple_statements_until(state, &TokenKind::RightBrace);
    let right_brace = utils::skip_right_brace(state);

    let body = FunctionBody {
        id: state.id(),
        span: Span::combine(left_brace, right_brace),
        comments: body_comments,
        left_brace,
        statements,
        right_brace,
    };

    StatementKind::Function(FunctionStatement {
        id: state.id(),
        span: Span::combine(function, body.span),
        comments,
        function,
        name,
        attributes,
        parameters,
        return_type,
        body,
        ampersand,
    })
}

pub fn method(state: &mut State, modifiers: MethodModifierGroup) -> Method {
    let comments = state.stream.comments();
    let attributes = state.get_attributes();
    let function = utils::skip(state, TokenKind::Function);

    let current = state.stream.current();
    let ampersand = if current.kind == TokenKind::Ampersand {
        state.stream.next();

        Some(current.span)
    } else {
        None
    };

    let name = identifiers::identifier_maybe_reserved(state);

    let symbol = &name.symbol;
    let is_constructor = symbol == b"__construct";

    if is_constructor {
        let parameters = parameters::constructor_parameter_list(state);

        return if state.stream.current().kind == TokenKind::LeftBrace {
            let body_comments = state.stream.comments();
            let left_brace = utils::skip_left_brace(state);
            let statements = blocks::multiple_statements_until(state, &TokenKind::RightBrace);
            let right_brace = utils::skip_right_brace(state);

            let body = MethodBody {
                id: state.id(),
                span: Span::combine(left_brace, right_brace),
                comments: body_comments,
                left_brace,
                statements,
                right_brace,
            };

            return Method::ConcreteConstructor(ConcreteConstructor {
                id: state.id(),
                span: Span::combine(function, body.span),
                comments,
                attributes,
                modifiers,
                function,
                ampersand,
                name,
                parameters,
                body,
            });
        } else {
            let semicolon = utils::skip_semicolon(state);

            Method::AbstractConstructor(AbstractConstructor {
                id: state.id(),
                span: Span::combine(function, semicolon),
                comments,
                attributes,
                modifiers,
                function,
                ampersand,
                name,
                parameters,
                semicolon,
            })
        };
    }

    let parameters = parameters::function_parameter_list(state);
    let return_type = if state.stream.current().kind == TokenKind::Colon {
        let colon = utils::skip_colon(state);
        let data_type = data_type::data_type(state);

        Some(ReturnType {
            id: state.id(),
            span: Span::combine(colon, data_type.span),
            colon,
            data_type,
        })
    } else {
        None
    };

    if state.stream.current().kind == TokenKind::LeftBrace {
        let body_comments = state.stream.comments();
        let left_brace = utils::skip_left_brace(state);
        let statements = blocks::multiple_statements_until(state, &TokenKind::RightBrace);
        let right_brace = utils::skip_right_brace(state);

        let body = MethodBody {
            id: state.id(),
            span: Span::combine(left_brace, right_brace),
            comments: body_comments,
            left_brace,
            statements,
            right_brace,
        };

        Method::Concrete(ConcreteMethod {
            id: state.id(),
            span: Span::combine(function, body.span),
            comments,
            attributes,
            modifiers,
            function,
            ampersand,
            name,
            parameters,
            return_type,
            body,
        })
    } else {
        let semicolon = utils::skip_semicolon(state);

        Method::Abstract(AbstractMethod {
            id: state.id(),
            span: Span::combine(function, semicolon),
            comments,
            attributes,
            modifiers,
            function,
            ampersand,
            name,
            parameters,
            return_type,
            semicolon,
        })
    }
}
