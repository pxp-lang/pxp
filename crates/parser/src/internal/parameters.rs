use crate::expressions;
use crate::internal::attributes;
use crate::internal::data_type;
use crate::internal::identifiers;
use crate::internal::modifiers;
use crate::internal::utils;
use crate::internal::variables;
use crate::state::State;
use crate::Parser;
use crate::ParserDiagnostic;
use pxp_ast::*;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_function_parameter_list(&mut self) -> FunctionParameterList {
        let comments = self.state.comments();
        let left_parenthesis = self.skip_left_parenthesis();
        let parameters = self.comma_separated(
            state,
            &|state| {
                attributes::gather_attributes();

                let ty = self.parse_optional_data_type();

                let mut current = self.current();
                let ampersand = if current.kind == TokenKind::Ampersand {
                    self.next();
                    current = self.current();
                    Some(current.span)
                } else {
                    None
                };

                let ellipsis = if current.kind == TokenKind::Ellipsis {
                    self.next();

                    Some(current.span)
                } else {
                    None
                };

                // 2. Then expect a variable.
                let var = self.parse_simple_variable();

                let mut default = None;
                if self.current_kind() == TokenKind::Equals {
                    self.next();
                    default = Some(self.parse_expression());
                }

                FunctionParameter {
                    id: self.state.id(),
                    // FIXME: This isn't taking other fields into account.
                    span: if ty.is_some() {
                        Span::combine(ty.span(), var.span)
                    } else {
                        var.span
                    },
                    comments: state.comments(),
                    name: var,
                    attributes: state.get_attributes(),
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
            id: self.state.id(),
            span: Span::combine(left_parenthesis, right_parenthesis),
            comments,
            left_parenthesis,
            parameters,
            right_parenthesis,
        }
    }

    pub fn parse_constructor_parameter_list(&mut self) -> ConstructorParameterList {
        let comments = self.state.comments();

        let left_parenthesis = self.skip_left_parenthesis();
        let parameters = self.comma_separated::<ConstructorParameter>(
            state,
            &|state| {
                attributes::gather_attributes();

                let modifiers = modifiers::collect_modifiers();
                let modifiers = modifiers::parse_promoted_property_group(modifiers);

                let ty = self.parse_optional_data_type();

                let mut current = self.current();
                let ampersand = if matches!(current.kind, TokenKind::Ampersand) {
                    self.next();

                    current = self.current();

                    Some(current.span)
                } else {
                    None
                };

                let (ellipsis, var) = if matches!(current.kind, TokenKind::Ellipsis) {
                    self.next();
                    let var = self.parse_simple_variable();
                    if !modifiers.is_empty() {
                        self.diagnostic(
                            ParserDiagnostic::PromotedPropertyCannotBeVariadic,
                            Severity::Error,
                            current.span,
                        );
                    }

                    (Some(current.span), var)
                } else {
                    (None, self.parse_simple_variable())
                };

                // 2. Then expect a variable.

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

                let mut default = None;
                if self.current_kind() == TokenKind::Equals {
                    self.next();
                    default = Some(self.parse_expression());
                }

                ConstructorParameter {
                    id: self.state.id(),
                    span: if ty.is_some() {
                        Span::combine(ty.span(), var.span)
                    } else {
                        var.span
                    },
                    comments: state.comments(),
                    name: var,
                    attributes: state.get_attributes(),
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
            id: self.state.id(),
            span: Span::combine(left_parenthesis, right_parenthesis),
            comments,
            left_parenthesis,
            parameters,
            right_parenthesis,
        }
    }

    pub fn parse_argument_list(&mut self) -> ArgumentList {
        let comments = self.state.comments();
        let start = self.skip_left_parenthesis();

        let mut arguments = Vec::new();
        let mut has_used_named_arguments = false;

        while !self.is_eof() && self.current_kind() != TokenKind::RightParen {
            let span = self.current_span();
            let (named, argument) = parse_argument();
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
            id: self.state.id(),
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
        let comments = self.state.comments();

        if self.current_kind() != TokenKind::LeftParen {
            return None;
        }

        let start = self.skip_left_parenthesis();

        let mut first_argument = None;

        while !self.is_eof() && self.current_kind() != TokenKind::RightParen {
            let span = self.current_span();
            let (named, argument) = parse_argument();
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
            id: self.state.id(),
            span: Span::combine(start, end),
            comments,
            left_parenthesis: start,
            right_parenthesis: end,
            argument: first_argument,
        })
    }

    fn parse_argument(&mut self) -> (bool, Argument) {
        if self.is_identifier_maybe_reserved(&self.current_kind())
            && state.peek().kind == TokenKind::Colon
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
                    id: self.state.id(),
                    span: Span::combine(name.span, value.span),
                    comments: state.comments(),
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
                id: self.state.id(),
                span: value.span,
                comments: state.comments(),
                ellipsis,
                value,
            }),
        )
    }
}
