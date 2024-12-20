use crate::internal::diagnostics::ParserDiagnostic;
use crate::Parser;
use pxp_ast::*;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
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

        let mut entries = vec![
            self.parse_property_entry(&modifiers),
        ];

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

    pub(crate) fn parse_hooked_property(&mut self) -> Property {
        todo!()
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
            modifiers: vec![PropertyModifier::Public(var)]
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
