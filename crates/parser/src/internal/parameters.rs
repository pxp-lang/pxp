use crate::internal::diagnostics::ParserDiagnostic;
use crate::Parser;
use pxp_ast::*;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_function_parameter_list(&mut self) -> FunctionParameterList {
        let comments = self.comments();
        let left_parenthesis = self.skip_left_parenthesis();
        let parameters = self.comma_separated(
            |parser| {
                parser.gather_attributes();

                let ty = parser.parse_optional_data_type();

                let ampersand = if parser.current_kind() == TokenKind::Ampersand {
                    Some(parser.next())
                } else {
                    None
                };

                let ellipsis = if parser.current_kind() == TokenKind::Ellipsis {
                    Some(parser.next())
                } else {
                    None
                };

                // 2. Then expect a variable.
                let var = parser.parse_simple_variable();

                let mut default = None;
                if parser.current_kind() == TokenKind::Equals {
                    parser.next();
                    default = Some(parser.parse_expression());
                }

                FunctionParameter {
                    id: parser.id(),
                    // FIXME: This isn't taking other fields into account.
                    span: if ty.is_some() {
                        Span::combine(ty.span(), var.span)
                    } else {
                        var.span
                    },
                    comments: parser.comments(),
                    name: var,
                    attributes: parser.get_attributes(),
                    data_type: ty,
                    ellipsis,
                    default,
                    ampersand,
                }
            },
            TokenKind::RightParen,
        );

        let right_parenthesis = self.skip_right_parenthesis();

        FunctionParameterList {
            id: self.id(),
            span: Span::combine(left_parenthesis, right_parenthesis),
            comments,
            left_parenthesis,
            parameters,
            right_parenthesis,
        }
    }

    pub fn parse_constructor_parameter_list(&mut self) -> ConstructorParameterList {
        let comments = self.comments();

        let left_parenthesis = self.skip_left_parenthesis();
        let parameters = self.comma_separated::<ConstructorParameter>(
            |parser| {
                parser.gather_attributes();

                let modifiers = parser.collect_modifiers();
                let modifiers = parser.parse_promoted_property_group(modifiers);

                let ty = parser.parse_optional_data_type();

                let ampersand = if parser.current_kind() == TokenKind::Ampersand {
                    Some(parser.next())
                } else {
                    None
                };

                let (ellipsis, var) = if parser.current_kind() == TokenKind::Ellipsis {
                    let ellipsis = parser.next();
                    let var = parser.parse_simple_variable();

                    if !modifiers.is_empty() {
                        parser.diagnostic(
                            ParserDiagnostic::PromotedPropertyCannotBeVariadic,
                            Severity::Error,
                            ellipsis,
                        );
                    }

                    (Some(ellipsis), var)
                } else {
                    (None, parser.parse_simple_variable())
                };

                // 2. Then expect a variable.

                if !modifiers.is_empty() {
                    match &ty {
                        Some(ty) => {
                            if ty.includes_callable() || ty.is_bottom() {
                                parser.diagnostic(
                                    ParserDiagnostic::ForbiddenTypeUsedInProperty,
                                    Severity::Error,
                                    ty.get_span(),
                                );
                            }
                        }
                        None => {
                            if let Some(modifier) = modifiers.get_readonly() {
                                parser.diagnostic(
                                    ParserDiagnostic::ReadonlyPropertyMustHaveType,
                                    Severity::Error,
                                    modifier.span(),
                                );
                            }
                        }
                    }
                }

                let mut default = None;
                if parser.current_kind() == TokenKind::Equals {
                    parser.next();
                    default = Some(parser.parse_expression());
                }

                ConstructorParameter {
                    id: parser.id(),
                    span: if ty.is_some() {
                        Span::combine(ty.span(), var.span)
                    } else {
                        var.span
                    },
                    comments: parser.comments(),
                    name: var,
                    attributes: parser.get_attributes(),
                    data_type: ty,
                    ellipsis,
                    default,
                    modifiers,
                    ampersand,
                }
            },
            TokenKind::RightParen,
        );

        let right_parenthesis = self.skip_right_parenthesis();

        ConstructorParameterList {
            id: self.id(),
            span: Span::combine(left_parenthesis, right_parenthesis),
            comments,
            left_parenthesis,
            parameters,
            right_parenthesis,
        }
    }

    pub fn parse_argument_list(&mut self) -> ArgumentList {
        let comments = self.comments();
        let start = self.skip_left_parenthesis();

        let mut arguments = Vec::new();
        let mut has_used_named_arguments = false;

        while !self.is_eof() && self.current_kind() != TokenKind::RightParen {
            let span = self.current_span();
            let (named, argument) = self.parse_argument();
            if named {
                has_used_named_arguments = true;
            } else if has_used_named_arguments {
                self.diagnostic(
                    ParserDiagnostic::CannotUsePositionalArgumentAfterNamedArgument,
                    Severity::Error,
                    span,
                );
            }

            arguments.push(argument);

            if self.current_kind() == TokenKind::Comma {
                self.next();
            } else {
                break;
            }
        }

        let end = self.skip_right_parenthesis();

        ArgumentList {
            id: self.id(),
            span: Span::combine(start, end),
            comments,
            left_parenthesis: start,
            right_parenthesis: end,
            arguments,
        }
    }

    pub fn parse_single_argument(
        &mut self,
        required: bool,
        only_positional: bool,
    ) -> Option<SingleArgument> {
        let comments = self.comments();

        if self.current_kind() != TokenKind::LeftParen {
            return None;
        }

        let start = self.skip_left_parenthesis();

        let mut first_argument = None;

        while !self.is_eof() && self.current_kind() != TokenKind::RightParen {
            let span = self.current_span();
            let (named, argument) = self.parse_argument();
            if only_positional && named {
                self.diagnostic(
                    ParserDiagnostic::PositionalArgumentsOnly,
                    Severity::Error,
                    span,
                );
            }

            if first_argument.is_some() {
                self.diagnostic(
                    ParserDiagnostic::OnlyAllowedOneArgument,
                    Severity::Error,
                    span,
                );
            }

            first_argument = Some(argument);

            if self.current_kind() == TokenKind::Comma {
                self.next();
            } else {
                break;
            }
        }

        if required && first_argument.is_none() {
            self.diagnostic(
                ParserDiagnostic::ArgumentRequired,
                Severity::Error,
                self.current_span(),
            );
        }

        let end = self.skip_right_parenthesis();

        Some(SingleArgument {
            id: self.id(),
            span: Span::combine(start, end),
            comments,
            left_parenthesis: start,
            right_parenthesis: end,
            argument: first_argument,
        })
    }

    fn parse_argument(&mut self) -> (bool, Argument) {
        if self.is_identifier_maybe_reserved(self.current_kind())
            && self.peek_kind() == TokenKind::Colon
        {
            let name = self.parse_identifier_maybe_reserved();
            let colon = self.skip(TokenKind::Colon);
            let ellipsis = if self.current_kind() == TokenKind::Ellipsis {
                Some(self.skip(TokenKind::Ellipsis))
            } else {
                None
            };
            let value = self.parse_expression();

            return (
                true,
                Argument::Named(NamedArgument {
                    id: self.id(),
                    span: Span::combine(name.span, value.span),
                    comments: self.comments(),
                    name,
                    colon,
                    ellipsis,
                    value,
                }),
            );
        }

        let ellipsis = if self.current_kind() == TokenKind::Ellipsis {
            Some(self.skip(TokenKind::Ellipsis))
        } else {
            None
        };

        let value = self.parse_expression();

        (
            false,
            Argument::Positional(PositionalArgument {
                id: self.id(),
                span: value.span,
                comments: self.comments(),
                ellipsis,
                value,
            }),
        )
    }
}
