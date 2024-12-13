use crate::expressions;
use crate::internal::blocks;
use crate::internal::data_type;
use crate::internal::identifiers;
use crate::internal::parameters;
use crate::internal::utils;
use crate::internal::variables;
use crate::state::State;
use crate::Parser;
use pxp_ast::Expression;
use pxp_ast::ExpressionKind;
use pxp_ast::StatementKind;
use pxp_ast::*;
use pxp_span::Span;
use pxp_token::TokenKind;

use super::names;

pub enum Method {
    Abstract(AbstractMethod),
    Concrete(ConcreteMethod),
    AbstractConstructor(AbstractConstructor),
    ConcreteConstructor(ConcreteConstructor),
}

impl<'a> Parser<'a> {
    pub fn parse_anonymous_function(&mut self) -> Expression {
        let comments = state.comments();
        let start_span = self.current().span;
        let attributes = state.get_attributes();
        let current = self.current();
        let r#static = if current.kind == TokenKind::Static {
            self.next();

            Some(current.span)
        } else {
            None
        };

        let function = self.skip(TokenKind::Function);

        let current = self.current();
        let ampersand = if current.kind == TokenKind::Ampersand {
            self.next();

            Some(current.span)
        } else {
            None
        };

        let parameters = parameters::parse_function_parameter_list();

        let current = self.current();
        let uses = if current.kind == TokenKind::Use {
            self.next();

            let left_parenthesis = self.skip_left_parenthesis();
            let variables = utils::comma_separated::<ClosureUseVariable>(
                state,
                &|state| {
                    let use_comments = state.comments();
                    let current = self.current();
                    let use_ampersand = if current.kind == TokenKind::Ampersand {
                        self.next();

                        Some(current.span)
                    } else {
                        None
                    };

                    let var = variables::parse_simple_variable();

                    ClosureUseVariable {
                        id: self.state.id(),
                        span: var.span,
                        comments: use_comments,
                        variable: var,
                        ampersand: use_ampersand,
                    }
                },
                TokenKind::RightParen,
            );

            let right_parenthesis = self.skip_right_parenthesis();

            Some(ClosureUse {
                id: self.state.id(),
                span: Span::combine(current.span, right_parenthesis),
                comments: state.comments(),
                r#use: current.span,
                left_parenthesis,
                variables,
                right_parenthesis,
            })
        } else {
            None
        };

        let return_type = if self.current().kind == TokenKind::Colon {
            let colon = utils::skip_colon();
            let data_type = data_type::parse_data_type();

            Some(ReturnType {
                id: self.state.id(),
                span: Span::combine(colon, data_type.span),
                colon,
                data_type,
            })
        } else {
            None
        };

        let body_comments = state.comments();
        let left_brace = utils::skip_left_brace();
        let statements = blocks::parse_multiple_statements_until(state, &TokenKind::RightBrace);
        let right_brace = utils::skip_right_brace();

        let body = FunctionBody {
            id: self.state.id(),
            span: Span::combine(left_brace, right_brace),
            comments: body_comments,
            left_brace,
            statements,
            right_brace,
        };

        let end_span = body.right_brace;

        Expression::new(
            self.state.id(),
            ExpressionKind::Closure(ClosureExpression {
                id: self.state.id(),
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

    pub fn parse_arrow_function(&mut self) -> Expression {
        let comments = state.comments();
        let start_span = self.current().span;
        let current = self.current();
        let r#static = if current.kind == TokenKind::Static {
            self.next();

            Some(current.span)
        } else {
            None
        };

        let r#fn = self.skip(TokenKind::Fn);

        let current = self.current();
        let ampersand = if self.current().kind == TokenKind::Ampersand {
            self.next();

            Some(current.span)
        } else {
            None
        };

        let attributes = state.get_attributes();
        let parameters = parameters::parse_function_parameter_list();
        let return_type = if self.current().kind == TokenKind::Colon {
            let colon = utils::skip_colon();
            let data_type = data_type::parse_data_type();

            Some(ReturnType {
                id: self.state.id(),
                span: Span::combine(colon, data_type.span),
                colon,
                data_type,
            })
        } else {
            None
        };

        let double_arrow = self.skip(TokenKind::DoubleArrow);

        let body = Box::new(self.parse_expression());
        let end_span = body.span;

        Expression::new(
            self.state.id(),
            ExpressionKind::ArrowFunction(ArrowFunctionExpression {
                id: self.state.id(),
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

    pub fn parse_function(&mut self) -> StatementKind {
        let comments = state.comments();

        let function = self.skip(TokenKind::Function);

        let current = self.current();
        let ampersand = if current.kind == TokenKind::Ampersand {
            self.next();

            Some(current.span)
        } else {
            None
        };

        let name = names::parse_type_name_maybe_soft_reserved();

        // get attributes before processing parameters, otherwise
        // parameters will steal attributes of this function.
        let attributes = state.get_attributes();

        let parameters = parameters::parse_function_parameter_list();
        let return_type = if self.current().kind == TokenKind::Colon {
            let colon = utils::skip_colon();
            let data_type = data_type::parse_data_type();

            Some(ReturnType {
                id: self.state.id(),
                span: Span::combine(colon, data_type.span),
                colon,
                data_type,
            })
        } else {
            None
        };

        let body_comments = state.comments();
        let left_brace = utils::skip_left_brace();
        let statements = blocks::parse_multiple_statements_until(state, &TokenKind::RightBrace);
        let right_brace = utils::skip_right_brace();

        let body = FunctionBody {
            id: self.state.id(),
            span: Span::combine(left_brace, right_brace),
            comments: body_comments,
            left_brace,
            statements,
            right_brace,
        };

        StatementKind::Function(FunctionStatement {
            id: self.state.id(),
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

    pub fn parse_method(&mut self, modifiers: MethodModifierGroup) -> Method {
        let comments = state.comments();
        let attributes = state.get_attributes();
        let function = self.skip(TokenKind::Function);

        let current = self.current();
        let ampersand = if current.kind == TokenKind::Ampersand {
            self.next();

            Some(current.span)
        } else {
            None
        };

        let name = identifiers::parse_identifier_maybe_reserved();

        let symbol = &name.symbol;
        let is_constructor = symbol == b"__construct";

        if is_constructor {
            let parameters = parameters::parse_constructor_parameter_list();

            return if self.current().kind == TokenKind::LeftBrace {
                let body_comments = state.comments();
                let left_brace = utils::skip_left_brace();
                let statements =
                    blocks::parse_multiple_statements_until(state, &TokenKind::RightBrace);
                let right_brace = utils::skip_right_brace();

                let body = MethodBody {
                    id: self.state.id(),
                    span: Span::combine(left_brace, right_brace),
                    comments: body_comments,
                    left_brace,
                    statements,
                    right_brace,
                };

                return Method::ConcreteConstructor(ConcreteConstructor {
                    id: self.state.id(),
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
                let semicolon = utils::skip_semicolon();

                Method::AbstractConstructor(AbstractConstructor {
                    id: self.state.id(),
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

        let parameters = parameters::parse_function_parameter_list();
        let return_type = if self.current().kind == TokenKind::Colon {
            let colon = utils::skip_colon();
            let data_type = data_type::parse_data_type();

            Some(ReturnType {
                id: self.state.id(),
                span: Span::combine(colon, data_type.span),
                colon,
                data_type,
            })
        } else {
            None
        };

        if self.current().kind == TokenKind::LeftBrace {
            let body_comments = state.comments();
            let left_brace = utils::skip_left_brace();
            let statements = blocks::parse_multiple_statements_until(state, &TokenKind::RightBrace);
            let right_brace = utils::skip_right_brace();

            let body = MethodBody {
                id: self.state.id(),
                span: Span::combine(left_brace, right_brace),
                comments: body_comments,
                left_brace,
                statements,
                right_brace,
            };

            Method::Concrete(ConcreteMethod {
                id: self.state.id(),
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
            let semicolon = utils::skip_semicolon();

            Method::Abstract(AbstractMethod {
                id: self.state.id(),
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
}
