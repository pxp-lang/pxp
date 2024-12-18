use crate::Parser;
use pxp_ast::Expression;
use pxp_ast::ExpressionKind;
use pxp_ast::StatementKind;
use pxp_ast::*;
use pxp_span::Span;
use pxp_token::TokenKind;

pub enum Method {
    Abstract(AbstractMethod),
    Concrete(ConcreteMethod),
    AbstractConstructor(AbstractConstructor),
    ConcreteConstructor(ConcreteConstructor),
}

impl<'a> Parser<'a> {
    pub fn parse_anonymous_function(&mut self) -> Expression {
        let comments = self.comments();
        let start_span = self.current_span();
        let attributes = self.get_attributes();
        let r#static = if self.current_kind() == TokenKind::Static {
            Some(self.next())
        } else {
            None
        };

        let function = self.skip(TokenKind::Function);

        let ampersand = if self.current_kind() == TokenKind::Ampersand {
            Some(self.next())
        } else {
            None
        };

        let parameters = self.parse_function_parameter_list();

        let uses = if self.current_kind() == TokenKind::Use {
            let r#use = self.next();
            let left_parenthesis = self.skip_left_parenthesis();
            let variables = self.comma_separated::<ClosureUseVariable>(
                |parser| {
                    let use_comments = parser.comments();
                    let use_ampersand = if parser.current_kind() == TokenKind::Ampersand {
                        Some(parser.next())
                    } else {
                        None
                    };

                    let var = parser.parse_simple_variable();

                    ClosureUseVariable {
                        id: parser.id(),
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
                id: self.id(),
                span: Span::combine(r#use, right_parenthesis),
                comments: self.comments(),
                r#use,
                left_parenthesis,
                variables,
                right_parenthesis,
            })
        } else {
            None
        };

        let return_type = if self.current_kind() == TokenKind::Colon {
            let colon = self.skip_colon();
            let data_type = self.parse_data_type();

            Some(ReturnType {
                id: self.id(),
                span: Span::combine(colon, data_type.span),
                colon,
                data_type,
            })
        } else {
            None
        };

        let body_comments = self.comments();
        let left_brace = self.skip_left_brace();
        let statements = self.parse_multiple_statements_until(TokenKind::RightBrace);
        let right_brace = self.skip_right_brace();

        let body = FunctionBody {
            id: self.id(),
            span: Span::combine(left_brace, right_brace),
            comments: body_comments,
            left_brace,
            statements,
            right_brace,
        };

        let end_span = body.right_brace;

        Expression::new(
            self.id(),
            ExpressionKind::Closure(ClosureExpression {
                id: self.id(),
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
        let comments = self.comments();
        let start_span = self.current_span();

        let r#static = if self.current_kind() == TokenKind::Static {
            Some(self.next())
        } else {
            None
        };

        let r#fn = self.skip(TokenKind::Fn);

        let ampersand = if self.current_kind() == TokenKind::Ampersand {
            Some(self.next())
        } else {
            None
        };

        let attributes = self.get_attributes();
        let parameters = self.parse_function_parameter_list();
        let return_type = if self.current_kind() == TokenKind::Colon {
            let colon = self.skip_colon();
            let data_type = self.parse_data_type();

            Some(ReturnType {
                id: self.id(),
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
            self.id(),
            ExpressionKind::ArrowFunction(ArrowFunctionExpression {
                id: self.id(),
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
        let comments = self.comments();

        let function = self.skip(TokenKind::Function);

        let ampersand = if self.current_kind() == TokenKind::Ampersand {
            Some(self.next())
        } else {
            None
        };

        let name = self.parse_type_name_maybe_soft_reserved();

        // get attributes before processing parameters, otherwise
        // parameters will steal attributes of this function.
        let attributes = self.get_attributes();

        let parameters = self.parse_function_parameter_list();
        let return_type = if self.current_kind() == TokenKind::Colon {
            let colon = self.skip_colon();
            let data_type = self.parse_data_type();

            Some(ReturnType {
                id: self.id(),
                span: Span::combine(colon, data_type.span),
                colon,
                data_type,
            })
        } else {
            None
        };

        let body_comments = self.comments();
        let left_brace = self.skip_left_brace();
        let statements = self.parse_multiple_statements_until(TokenKind::RightBrace);
        let right_brace = self.skip_right_brace();

        let body = FunctionBody {
            id: self.id(),
            span: Span::combine(left_brace, right_brace),
            comments: body_comments,
            left_brace,
            statements,
            right_brace,
        };

        StatementKind::Function(FunctionStatement {
            id: self.id(),
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
        let comments = self.comments();
        let attributes = self.get_attributes();
        let function = self.skip(TokenKind::Function);

        let ampersand = if self.current_kind() == TokenKind::Ampersand {
            Some(self.next())
        } else {
            None
        };

        let name = self.parse_identifier_maybe_reserved();

        let symbol = &name.symbol;
        let is_constructor = symbol == b"__construct";

        if is_constructor {
            let parameters = self.parse_constructor_parameter_list();

            return if self.current_kind() == TokenKind::LeftBrace {
                let body_comments = self.comments();
                let left_brace = self.skip_left_brace();
                let statements = self.parse_multiple_statements_until(TokenKind::RightBrace);
                let right_brace = self.skip_right_brace();

                let body = MethodBody {
                    id: self.id(),
                    span: Span::combine(left_brace, right_brace),
                    comments: body_comments,
                    left_brace,
                    statements,
                    right_brace,
                };

                return Method::ConcreteConstructor(ConcreteConstructor {
                    id: self.id(),
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
                let semicolon = self.skip_semicolon();

                Method::AbstractConstructor(AbstractConstructor {
                    id: self.id(),
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

        let parameters = self.parse_function_parameter_list();
        let return_type = if self.current_kind() == TokenKind::Colon {
            let colon = self.skip_colon();
            let data_type = self.parse_data_type();

            Some(ReturnType {
                id: self.id(),
                span: Span::combine(colon, data_type.span),
                colon,
                data_type,
            })
        } else {
            None
        };

        if self.current_kind() == TokenKind::LeftBrace {
            let body_comments = self.comments();
            let left_brace = self.skip_left_brace();
            let statements = self.parse_multiple_statements_until(TokenKind::RightBrace);
            let right_brace = self.skip_right_brace();

            let body = MethodBody {
                id: self.id(),
                span: Span::combine(left_brace, right_brace),
                comments: body_comments,
                left_brace,
                statements,
                right_brace,
            };

            Method::Concrete(ConcreteMethod {
                id: self.id(),
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
            let semicolon = self.skip_semicolon();

            Method::Abstract(AbstractMethod {
                id: self.id(),
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
