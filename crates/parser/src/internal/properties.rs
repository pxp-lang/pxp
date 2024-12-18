use crate::Parser;
use crate::internal::diagnostics::ParserDiagnostic;
use pxp_ast::*;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_property(&mut self, modifiers: PropertyModifierGroup) -> Property {
        let ty = self.parse_optional_data_type();

        let mut entries = vec![];
        let mut type_checked = false;
        loop {
            let variable = self.parse_simple_variable();

            if !type_checked {
                type_checked = true;
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
            }

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

                entries.push(PropertyEntry {
                    id: self.id(),
                    span,
                    kind: PropertyEntryKind::Initialized(InitializedPropertyEntry {
                        id: self.id(),
                        span,
                        variable,
                        equals,
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

        Property {
            id: self.id(),
            span: if ty.is_some() {
                Span::combine(ty.span(), end)
            } else {
                entries.span()
            },
            r#type: ty,
            modifiers,
            attributes: self.get_attributes(),
            entries,
            end,
        }
    }

    pub fn parse_var_property(&mut self) -> VariableProperty {
        self.skip(TokenKind::Var);

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

        VariableProperty {
            id: self.id(),
            span: if ty.is_some() {
                Span::combine(ty.span(), end)
            } else {
                entries.span()
            },
            r#type: ty,
            attributes: self.get_attributes(),
            entries,
            end,
        }
    }
}
