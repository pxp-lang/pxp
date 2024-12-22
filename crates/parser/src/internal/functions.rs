use crate::Parser;
use pxp_ast::Expression;
use pxp_ast::ExpressionKind;
use pxp_ast::StatementKind;
use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

use super::diagnostics::ParserDiagnostic;

impl<'a> Parser<'a> {
    pub(crate) fn parse_anonymous_function(&mut self) -> Expression {
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

        let return_type = self.parse_return_type();
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
            ExpressionKind::Closure(Box::new(ClosureExpression {
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
            })),
            Span::new(start_span.start, end_span.end),
            CommentGroup::default(),
        )
    }

    pub(crate) fn parse_arrow_function(&mut self) -> Expression {
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
        let return_type = self.parse_return_type();
        let double_arrow = self.skip(TokenKind::DoubleArrow);

        let body = Box::new(self.parse_expression());
        let end_span = body.span;

        Expression::new(
            self.id(),
            ExpressionKind::ArrowFunction(Box::new(ArrowFunctionExpression {
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
            })),
            Span::new(start_span.start, end_span.end),
            CommentGroup::default(),
        )
    }

    pub(crate) fn parse_function(&mut self) -> StatementKind {
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
        let return_type = self.parse_return_type();

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

        StatementKind::Function(Box::new(FunctionStatement {
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
        }))
    }

    pub(crate) fn parse_method(&mut self, modifiers: MethodModifierGroup) -> Method {
        let comments = self.comments();
        let attributes = self.get_attributes();
        let function = self.expect(TokenKind::Function);
        let ampersand = if self.current_kind() == TokenKind::Ampersand {
            Some(self.next())
        } else {
            None
        };
        let name = self.parse_identifier_maybe_reserved();
        let parameters = self.parse_method_parameter_list();
        let return_type = self.parse_return_type();
        let body = self.parse_method_body();

        Method {
            id: self.id(),
            span: modifiers.span.join(body.span),
            comments,
            attributes,
            modifiers,
            function,
            ampersand,
            name,
            parameters,
            return_type,
            body,
        }
    }

    fn parse_abstract_method_body(&mut self) -> MethodBodyKind {
        let semicolon = self.expect(TokenKind::SemiColon);

        MethodBodyKind::Abstract(AbstractMethodBody {
            id: self.id(),
            span: semicolon,
            semicolon,
        })
    }

    fn parse_concrete_method_body(&mut self) -> MethodBodyKind {
        let left_brace = self.expect(TokenKind::LeftBrace);
        let statements = self.parse_multiple_statements_until(TokenKind::RightBrace);
        let right_brace = self.expect(TokenKind::RightBrace);

        MethodBodyKind::Concrete(ConcreteMethodBody {
            id: self.id(),
            span: Span::combine(left_brace, right_brace),
            left_brace,
            statements,
            right_brace,
        })
    }

    fn parse_method_body(&mut self) -> MethodBody {
        let kind = match self.current_kind() {
            TokenKind::SemiColon => self.parse_abstract_method_body(),
            TokenKind::LeftBrace => self.parse_concrete_method_body(),
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::SemiColon, TokenKind::LeftBrace],
                        found: self.current().to_owned(),
                    },
                    Severity::Error,
                    self.current_span(),
                );

                MethodBodyKind::Missing(self.current_span())
            }
        };

        MethodBody {
            id: self.id(),
            span: kind.span(),
            kind,
        }
    }

    fn parse_method_parameter_list(&mut self) -> MethodParameterList {
        let left_parenthesis = self.expect(TokenKind::LeftParen);
        let parameters = self.comma_separated(
            |parser| parser.parse_method_parameter(),
            TokenKind::RightParen,
        );
        let right_parenthesis = self.expect(TokenKind::RightParen);

        MethodParameterList {
            id: self.id(),
            span: Span::combine(left_parenthesis, right_parenthesis),
            left_parenthesis,
            parameters,
            right_parenthesis,
        }
    }

    fn parse_method_parameter(&mut self) -> MethodParameter {
        self.gather_attributes();

        let attributes = self.get_attributes();
        let modifiers = self.collect_modifiers();
        let modifiers = self.parse_promoted_property_group(modifiers);

        let ty = self.parse_optional_data_type();
        let ampersand = if self.current_kind() == TokenKind::Ampersand {
            Some(self.next())
        } else {
            None
        };

        let (ellipsis, name) = if self.current_kind() == TokenKind::Ellipsis {
            let ellipsis = self.next();
            let var = self.parse_simple_variable();

            if !modifiers.is_empty() {
                self.diagnostic(
                    ParserDiagnostic::PromotedPropertyCannotBeVariadic,
                    Severity::Error,
                    ellipsis,
                );
            }

            (Some(ellipsis), var)
        } else {
            (None, self.parse_simple_variable())
        };

        if !modifiers.is_empty() {
            match &ty {
                Some(ty) => {
                    if ty.includes_callable() || ty.is_bottom() {
                        self.diagnostic(
                            ParserDiagnostic::ForbiddenTypeUsedInProperty,
                            Severity::Error,
                            ty.get_span(),
                        );
                    }
                }
                None => {
                    if let Some(modifier) = modifiers.get_readonly() {
                        self.diagnostic(
                            ParserDiagnostic::ReadonlyPropertyMustHaveType,
                            Severity::Error,
                            modifier.span(),
                        );
                    }
                }
            }
        }

        let default = if self.current_kind() == TokenKind::Equals {
            self.next();

            Some(self.parse_expression())
        } else {
            None
        };

        let start = if !modifiers.is_empty() {
            modifiers.span
        } else if ty.is_some() {
            ty.span()
        } else {
            name.span
        };

        let end = if default.is_some() {
            default.span()
        } else {
            name.span
        };

        MethodParameter {
            id: self.id(),
            span: start.join(end),
            modifiers: if modifiers.is_empty() {
                None
            } else {
                Some(modifiers)
            },
            name,
            attributes,
            data_type: ty,
            ellipsis,
            default,
            ampersand,
        }
    }

    fn parse_return_type(&mut self) -> Option<ReturnType> {
        if self.current_kind() != TokenKind::Colon {
            return None;
        }

        let colon = self.expect(TokenKind::Colon);
        let data_type = self.parse_data_type();

        Some(ReturnType {
            id: self.id(),
            span: Span::combine(colon, data_type.span),
            colon,
            data_type,
        })
    }

    // pub(crate) fn parse_method(&mut self, modifiers: MethodModifierGroup) -> Method {
    //     let comments = self.comments();
    //     let attributes = self.get_attributes();
    //     let function = self.skip(TokenKind::Function);

    //     let ampersand = if self.current_kind() == TokenKind::Ampersand {
    //         Some(self.next())
    //     } else {
    //         None
    //     };

    //     let name = self.parse_identifier_maybe_reserved();

    //     let symbol = &name.symbol;
    //     let is_constructor = symbol == b"__construct";

    //     if is_constructor {
    //         let parameters = self.parse_constructor_parameter_list();

    //         return if self.current_kind() == TokenKind::LeftBrace {
    //             let body_comments = self.comments();
    //             let left_brace = self.skip_left_brace();
    //             let statements = self.parse_multiple_statements_until(TokenKind::RightBrace);
    //             let right_brace = self.skip_right_brace();

    //             let body = MethodBody {
    //                 id: self.id(),
    //                 span: Span::combine(left_brace, right_brace),
    //                 comments: body_comments,
    //                 left_brace,
    //                 statements,
    //                 right_brace,
    //             };

    //             return Method::ConcreteConstructor(ConcreteConstructor {
    //                 id: self.id(),
    //                 span: Span::combine(function, body.span),
    //                 comments,
    //                 attributes,
    //                 modifiers,
    //                 function,
    //                 ampersand,
    //                 name,
    //                 parameters,
    //                 body,
    //             });
    //         } else {
    //             let semicolon = self.skip_semicolon();

    //             Method::AbstractConstructor(AbstractConstructor {
    //                 id: self.id(),
    //                 span: Span::combine(function, semicolon),
    //                 comments,
    //                 attributes,
    //                 modifiers,
    //                 function,
    //                 ampersand,
    //                 name,
    //                 parameters,
    //                 semicolon,
    //             })
    //         };
    //     }

    //     let parameters = self.parse_function_parameter_list();
    //     let return_type = if self.current_kind() == TokenKind::Colon {
    //         let colon = self.skip_colon();
    //         let data_type = self.parse_data_type();

    //         Some(ReturnType {
    //             id: self.id(),
    //             span: Span::combine(colon, data_type.span),
    //             colon,
    //             data_type,
    //         })
    //     } else {
    //         None
    //     };

    //     if self.current_kind() == TokenKind::LeftBrace {
    //         let body_comments = self.comments();
    //         let left_brace = self.skip_left_brace();
    //         let statements = self.parse_multiple_statements_until(TokenKind::RightBrace);
    //         let right_brace = self.skip_right_brace();

    //         let body = MethodBody {
    //             id: self.id(),
    //             span: Span::combine(left_brace, right_brace),
    //             comments: body_comments,
    //             left_brace,
    //             statements,
    //             right_brace,
    //         };

    //         Method::Concrete(ConcreteMethod {
    //             id: self.id(),
    //             span: Span::combine(function, body.span),
    //             comments,
    //             attributes,
    //             modifiers,
    //             function,
    //             ampersand,
    //             name,
    //             parameters,
    //             return_type,
    //             body,
    //         })
    //     } else {
    //         let semicolon = self.skip_semicolon();

    //         Method::Abstract(AbstractMethod {
    //             id: self.id(),
    //             span: Span::combine(function, semicolon),
    //             comments,
    //             attributes,
    //             modifiers,
    //             function,
    //             ampersand,
    //             name,
    //             parameters,
    //             return_type,
    //             semicolon,
    //         })
    //     }
    // }
}
