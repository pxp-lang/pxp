use crate::ParserDiagnostic;
use crate::{state::State, Parser};
use pxp_ast::*;

use pxp_diagnostics::Severity;
use pxp_span::{Span, Spanned};
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    #[inline(always)]
    pub fn parse_class_group(&mut self, input: Vec<(Span, TokenKind)>) -> ClassModifierGroup {
        let modifiers = input
            .iter()
            .filter_map(|(span, token)| match token {
                TokenKind::Readonly => Some(ClassModifier::Readonly(*span)),
                TokenKind::Final => Some(ClassModifier::Final(*span)),
                TokenKind::Abstract => Some(ClassModifier::Abstract(*span)),
                _ => {
                    self.diagnostic(
                        ParserDiagnostic::InvalidClassModifier,
                        Severity::Error,
                        *span,
                    );

                    None
                }
            })
            .collect::<Vec<ClassModifier>>();

        let group = ClassModifierGroup {
            id: self.state.id(),
            span: modifiers.span(),
            modifiers,
        };

        if group.has_abstract() && group.has_final() {
            let start = input.first().unwrap().0;
            let end = input.last().unwrap().0;
            let span = Span::new(start.start, end.end);

            self.diagnostic(
                ParserDiagnostic::CannotUseFinalWithAbstract,
                Severity::Error,
                span,
            );
        }

        group
    }

    #[inline(always)]
    pub fn parse_method_group(&mut self, input: Vec<(Span, TokenKind)>) -> MethodModifierGroup {
        let modifiers = input
            .iter()
            .filter_map(|(span, token)| match token {
                TokenKind::Final => Some(MethodModifier::Final(*span)),
                TokenKind::Abstract => Some(MethodModifier::Abstract(*span)),
                TokenKind::Private => Some(MethodModifier::Private(*span)),
                TokenKind::Protected => Some(MethodModifier::Protected(*span)),
                TokenKind::Public => Some(MethodModifier::Public(*span)),
                TokenKind::Static => Some(MethodModifier::Static(*span)),
                _ => {
                    self.diagnostic(
                        ParserDiagnostic::InvalidMethodModifier,
                        Severity::Error,
                        *span,
                    );

                    None
                }
            })
            .collect::<Vec<MethodModifier>>();

        let group = MethodModifierGroup {
            id: self.state.id(),
            span: modifiers.span(),
            modifiers,
        };

        if group.has_abstract() && group.has_final() {
            let start = input.first().unwrap().0;
            let end = input.last().unwrap().0;
            let span = Span::new(start.start, end.end);

            self.diagnostic(
                ParserDiagnostic::CannotUseFinalWithAbstract,
                Severity::Error,
                span,
            );
        }

        group
    }

    #[inline(always)]
    pub fn parse_property_group(&mut self, input: Vec<(Span, TokenKind)>) -> PropertyModifierGroup {
        let modifiers = input
            .iter()
            .filter_map(|(span, token)| match token {
                TokenKind::Readonly => Some(PropertyModifier::Readonly(*span)),
                TokenKind::Static => Some(PropertyModifier::Static(*span)),
                TokenKind::Public => Some(PropertyModifier::Public(*span)),
                TokenKind::Protected => Some(PropertyModifier::Protected(*span)),
                TokenKind::Private => Some(PropertyModifier::Private(*span)),
                _ => {
                    self.diagnostic(
                        ParserDiagnostic::InvalidPropertyModifier,
                        Severity::Error,
                        *span,
                    );

                    None
                }
            })
            .collect::<Vec<PropertyModifier>>();

        PropertyModifierGroup {
            id: self.state.id(),
            span: modifiers.span(),
            modifiers,
        }
    }

    #[inline(always)]
    pub fn parse_promoted_property_group(
        &mut self,
        input: Vec<(Span, TokenKind)>,
    ) -> PromotedPropertyModifierGroup {
        let modifiers = input
            .iter()
            .filter_map(|(span, token)| match token {
                TokenKind::Readonly => Some(PromotedPropertyModifier::Readonly(*span)),
                TokenKind::Private => Some(PromotedPropertyModifier::Private(*span)),
                TokenKind::Protected => Some(PromotedPropertyModifier::Protected(*span)),
                TokenKind::Public => Some(PromotedPropertyModifier::Public(*span)),
                _ => {
                    self.diagnostic(
                        ParserDiagnostic::InvalidPropertyModifier,
                        Severity::Error,
                        *span,
                    );

                    None
                }
            })
            .collect::<Vec<PromotedPropertyModifier>>();

        PromotedPropertyModifierGroup {
            id: self.state.id(),
            span: modifiers.span(),
            modifiers,
        }
    }

    pub fn parse_constant_group(&mut self, input: Vec<(Span, TokenKind)>) -> ConstantModifierGroup {
        let modifiers = input
            .iter()
            .filter_map(|(span, token)| match token {
                TokenKind::Protected => Some(ConstantModifier::Protected(*span)),
                TokenKind::Public => Some(ConstantModifier::Public(*span)),
                TokenKind::Private => Some(ConstantModifier::Private(*span)),
                TokenKind::Final => Some(ConstantModifier::Final(*span)),
                _ => {
                    self.diagnostic(
                        ParserDiagnostic::InvalidConstantModifier,
                        Severity::Error,
                        *span,
                    );

                    None
                }
            })
            .collect::<Vec<ConstantModifier>>();

        let group = ConstantModifierGroup {
            id: self.state.id(),
            span: modifiers.span(),
            modifiers,
        };

        if group.has_final() && group.has_private() {
            let start = input.first().unwrap().0;
            let end = input.last().unwrap().0;
            let span = Span::new(start.start, end.end);

            self.diagnostic(
                ParserDiagnostic::CannotUseFinalWithPrivateOnConstant,
                Severity::Error,
                span,
            );
        }

        group
    }

    pub fn collect_modifiers(&mut self) -> Vec<(Span, TokenKind)> {
        let mut collected: Vec<(Span, TokenKind)> = vec![];

        let collectable_tokens = [
            TokenKind::Private,
            TokenKind::Protected,
            TokenKind::Public,
            TokenKind::Final,
            TokenKind::Abstract,
            TokenKind::Static,
            TokenKind::Readonly,
        ];

        let mut current = self.current().clone();
        let mut current_kind = current.kind;
        let mut current_span = current.span;

        while collectable_tokens.contains(&current_kind) {
            if let Some((span, _)) = collected.iter().find(|(_, kind)| kind == &current_kind) {
                self.diagnostic(ParserDiagnostic::DuplicateModifier, Severity::Error, *span);
            }

            // guard against multiple visibility modifiers, we don't care where these modifiers are used.
            if matches!(
                current_kind,
                TokenKind::Public | TokenKind::Protected | TokenKind::Private
            ) {
                if let Some((span, _)) = collected.iter().find(|(_, kind)| {
                    matches!(
                        kind,
                        TokenKind::Public | TokenKind::Protected | TokenKind::Private
                    ) && kind != &current_kind
                }) {
                    self.diagnostic(
                        ParserDiagnostic::MultipleVisibilityModifiers,
                        Severity::Error,
                        *span,
                    );
                }
            }

            collected.push((current_span, current_kind));

            self.next();

            current = self.current().clone();
            current_kind = current.kind;
            current_span = current.span;
        }

        collected
    }
}
