use crate::internal::diagnostics::ParserDiagnostic;
use crate::Parser;
use pxp_ast::*;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::IsSpanned;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub(crate) fn parse_property(&mut self, modifiers: PropertyModifierGroup) -> Property {
        let ty = self.parse_optional_data_type();

        if modifiers.has_readonly() && modifiers.has_static() {
            self.diagnostic(
                ParserDiagnostic::StaticPropertyCannotBeReadonly,
                Severity::Error,
                self.current_span(),
            );
        }

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

        let entry = self.parse_property_entry(&modifiers);

        if self.current_kind() == TokenKind::LeftBrace {
            return self.parse_hooked_property(modifiers, ty, entry);
        }

        let mut entries = vec![entry];

        while !self.is_eof() && self.current_kind() != TokenKind::SemiColon {
            entries.push(self.parse_property_entry(&modifiers));

            if self.current_kind() == TokenKind::Comma {
                self.next();
            } else {
                break;
            }
        }

        let end = self.skip_semicolon();

        Property::Simple(SimpleProperty {
            id: self.id(),
            span: if ty.is_some() {
                Span::combine(ty.span(), end)
            } else {
                entries.span()
            },
            var: None,
            r#type: ty,
            modifiers,
            attributes: self.get_attributes(),
            entries,
            semicolon: end,
        })
    }

    fn parse_property_entry(&mut self, modifiers: &PropertyModifierGroup) -> PropertyEntry {
        let variable = self.parse_simple_variable();

        if self.current_kind() == TokenKind::Equals {
            if let Some(modifier) = modifiers.get_readonly() {
                self.diagnostic(
                    ParserDiagnostic::ReadonlyPropertyCannotHaveDefaultValue,
                    Severity::Error,
                    modifier.span(),
                );
            }

            let equals = self.next();
            let value = self.parse_expression();
            let span = Span::combine(variable.span, value.span);

            PropertyEntry {
                id: self.id(),
                span,
                kind: PropertyEntryKind::Initialized(InitializedPropertyEntry {
                    id: self.id(),
                    span,
                    variable,
                    equals,
                    value,
                }),
            }
        } else {
            PropertyEntry {
                id: self.id(),
                span: variable.span,
                kind: PropertyEntryKind::Uninitialized(UninitializedPropertyEntry {
                    id: self.id(),
                    span: variable.span,
                    variable,
                }),
            }
        }
    }

    fn parse_hooked_property(
        &mut self,
        modifiers: PropertyModifierGroup,
        r#type: Option<DataType>,
        entry: PropertyEntry,
    ) -> Property {
        let left_brace = self.skip_left_brace();
        let mut hooks = vec![];

        while !self.is_eof() && self.current_kind() != TokenKind::RightBrace {
            hooks.push(self.parse_property_hook());
        }

        if hooks.is_empty() {
            self.diagnostic(
                ParserDiagnostic::ExpectedPropertyHook,
                Severity::Error,
                left_brace,
            );
        }

        let right_brace = self.skip_right_brace();
        let span = left_brace.join(right_brace);
        let hooks = PropertyHookList {
            id: self.id(),
            span,
            left_brace,
            hooks,
            right_brace,
        };

        Property::Hooked(HookedProperty {
            id: self.id(),
            span: Span::combine(modifiers.span(), hooks.span),
            attributes: self.get_attributes(),
            modifiers,
            r#type,
            entry,
            hooks,
        })
    }

    fn parse_property_hook(&mut self) -> PropertyHook {
        let kind = match self.current_kind() {
            TokenKind::Identifier if self.current_symbol() == b"get" => {
                PropertyHookKind::Get(self.next())
            }
            TokenKind::Identifier if self.current_symbol() == b"set" => {
                PropertyHookKind::Set(self.next())
            }
            TokenKind::Identifier => {
                self.diagnostic(
                    ParserDiagnostic::InvalidPropertyHook,
                    Severity::Error,
                    self.current_span(),
                );

                PropertyHookKind::Invalid(self.next())
            }
            _ => {
                self.diagnostic(
                    ParserDiagnostic::InvalidPropertyHook,
                    Severity::Error,
                    self.current_span(),
                );

                PropertyHookKind::Invalid(self.current_span())
            }
        };

        let parameters = if self.current_kind() == TokenKind::LeftParen {
            Some(self.parse_function_parameter_list())
        } else {
            None
        };

        let body = self.parse_property_hook_body();

        PropertyHook {
            id: self.id(),
            span: kind.span().join(body.span()),
            kind,
            parameters,
            body,
        }
    }

    fn parse_property_hook_body(&mut self) -> PropertyHookBody {
        match self.current_kind() {
            TokenKind::SemiColon => PropertyHookBody::Abstract(self.next()),
            TokenKind::DoubleArrow => {
                let double_arrow = self.next();
                let expression = self.parse_expression();

                PropertyHookBody::Concrete(ConcretePropertyHookBody::Expression(
                    ConcretePropertyHookBodyExpression {
                        id: self.id(),
                        span: double_arrow.join(expression.span),
                        arrow: double_arrow,
                        expression,
                        semicolon: self.skip_semicolon(),
                    },
                ))
            }
            TokenKind::LeftBrace => {
                let left_brace = self.next();
                let mut statements = vec![];

                while !self.is_eof() && self.current_kind() != TokenKind::RightBrace {
                    statements.push(self.parse_statement());
                }

                let right_brace = self.skip_right_brace();

                PropertyHookBody::Concrete(ConcretePropertyHookBody::Block(
                    ConcretePropertyHookBodyBlock {
                        id: self.id(),
                        span: left_brace.join(right_brace),
                        left_brace,
                        right_brace,
                        body: statements,
                    },
                ))
            }
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::LeftBrace, TokenKind::DoubleArrow],
                        found: self.current().to_owned(),
                    },
                    Severity::Error,
                    self.current_span(),
                );

                PropertyHookBody::Invalid(self.current_span())
            }
        }
    }

    pub(crate) fn parse_var_property(&mut self) -> Property {
        let var = self.skip(TokenKind::Var);
        let ty = self.parse_optional_data_type();

        let mut entries: Vec<PropertyEntry> = vec![];
        let mut type_checked = false;
        loop {
            let variable = self.parse_simple_variable();

            if !type_checked {
                type_checked = true;

                if let Some(ty) = &ty {
                    if ty.includes_callable() || ty.is_bottom() {
                        self.diagnostic(
                            ParserDiagnostic::ForbiddenTypeUsedInProperty,
                            Severity::Error,
                            ty.get_span(),
                        );
                    }
                }
            }

            let current = self.current();
            if current.kind == TokenKind::Equals {
                self.next();
                let value = self.parse_expression();
                let span = Span::combine(variable.span, value.span);

                entries.push(PropertyEntry {
                    id: self.id(),
                    span,
                    kind: PropertyEntryKind::Initialized(InitializedPropertyEntry {
                        id: self.id(),
                        span,
                        variable,
                        equals: span,
                        value,
                    }),
                });
            } else {
                entries.push(PropertyEntry {
                    id: self.id(),
                    span: variable.span,
                    kind: PropertyEntryKind::Uninitialized(UninitializedPropertyEntry {
                        id: self.id(),
                        span: variable.span,
                        variable,
                    }),
                });
            }

            if self.current_kind() == TokenKind::Comma {
                self.next();
            } else {
                break;
            }
        }

        let end = self.skip_semicolon();
        let modifiers = PropertyModifierGroup {
            id: self.id(),
            span: var,
            modifiers: vec![PropertyModifier::Public(var)],
        };

        Property::Simple(SimpleProperty {
            id: self.id(),
            span: if ty.is_some() {
                Span::combine(ty.span(), end)
            } else {
                entries.span()
            },
            modifiers,
            var: Some(var),
            r#type: ty,
            attributes: self.get_attributes(),
            entries,
            semicolon: end,
        })
    }
}
