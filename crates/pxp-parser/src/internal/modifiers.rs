use crate::state::State;
use pxp_ast::modifiers::ClassModifier;
use pxp_ast::modifiers::ClassModifierGroup;
use pxp_ast::modifiers::ConstantModifier;
use pxp_ast::modifiers::ConstantModifierGroup;
use pxp_ast::modifiers::MethodModifier;
use pxp_ast::modifiers::MethodModifierGroup;
use pxp_ast::modifiers::PromotedPropertyModifier;
use pxp_ast::modifiers::PromotedPropertyModifierGroup;
use pxp_ast::modifiers::PropertyModifier;
use pxp_ast::modifiers::PropertyModifierGroup;
use pxp_diagnostics::DiagnosticKind;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;

#[inline(always)]
pub fn class_group(state: &mut State, input: Vec<(Span, TokenKind)>) -> ClassModifierGroup {
    let modifiers = input
        .iter()
        .filter_map(|(span, token)| match token {
            TokenKind::Readonly => Some(ClassModifier::Readonly(*span)),
            TokenKind::Final => Some(ClassModifier::Final(*span)),
            TokenKind::Abstract => Some(ClassModifier::Abstract(*span)),
            _ => {
                state.diagnostic(
                    DiagnosticKind::InvalidClassModifier,
                    Severity::Error,
                    *span
                );

                None
            }
        })
        .collect::<Vec<ClassModifier>>();

    let group = ClassModifierGroup { modifiers };

    if group.has_abstract() && group.has_final() {
        let start = input.first().unwrap().0;
        let end = input.last().unwrap().0;
        let span = Span::new(start.start, end.end);

        state.diagnostic(
            DiagnosticKind::CannotUseFinalWithAbstract,
            Severity::Error,
            span
        );
    }

    group
}

#[inline(always)]
pub fn method_group(state: &mut State, input: Vec<(Span, TokenKind)>) -> MethodModifierGroup {
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
                state.diagnostic(
                    DiagnosticKind::InvalidMethodModifier,
                    Severity::Error,
                    *span
                );

                None
            }
        })
        .collect::<Vec<MethodModifier>>();

    let group = MethodModifierGroup { modifiers };

    if group.has_abstract() && group.has_final() {
        let start = input.first().unwrap().0;
        let end = input.last().unwrap().0;
        let span = Span::new(start.start, end.end);

        state.diagnostic(
            DiagnosticKind::CannotUseFinalWithAbstract,
            Severity::Error,
            span
        );
    }

    group
}

#[inline(always)]
pub fn property_group(input: Vec<(Span, TokenKind)>) -> PropertyModifierGroup {
    let modifiers = input
        .iter()
        .map(|(span, token)| match token {
            TokenKind::Readonly => PropertyModifier::Readonly(*span),
            TokenKind::Static => PropertyModifier::Static(*span),
            TokenKind::Public => PropertyModifier::Public(*span),
            TokenKind::Protected => PropertyModifier::Protected(*span),
            TokenKind::Private => PropertyModifier::Private(*span),
            _ => todo!("tolerant mode"), /*Err(error::modifier_cannot_be_used_for_property(
                                             token.to_string(),
                                             *span,
                                         ))*/
        })
        .collect::<Vec<PropertyModifier>>();

    PropertyModifierGroup { modifiers }
}

#[inline(always)]
pub fn promoted_property_group(input: Vec<(Span, TokenKind)>) -> PromotedPropertyModifierGroup {
    let modifiers = input
        .iter()
        .map(|(span, token)| match token {
            TokenKind::Readonly => PromotedPropertyModifier::Readonly(*span),
            TokenKind::Private => PromotedPropertyModifier::Private(*span),
            TokenKind::Protected => PromotedPropertyModifier::Protected(*span),
            TokenKind::Public => PromotedPropertyModifier::Public(*span),
            _ => todo!("tolerant mode"), /*Err(error::modifier_cannot_be_used_for_promoted_property(
                                             token.to_string(),
                                             *span,
                                         ))*/
        })
        .collect::<Vec<PromotedPropertyModifier>>();

    PromotedPropertyModifierGroup { modifiers }
}

pub fn constant_group(input: Vec<(Span, TokenKind)>) -> ConstantModifierGroup {
    let mut final_span = None;
    let mut private_span = None;

    let modifiers = input
        .iter()
        .map(|(span, token)| match token {
            TokenKind::Protected => ConstantModifier::Protected(*span),
            TokenKind::Public => ConstantModifier::Public(*span),
            TokenKind::Private => {
                private_span = Some(*span);
                if let Some(final_span) = final_span {
                    todo!("tolerant mode")
                    // Err(error::final_and_private_modifiers_combined_for_constant(
                    //     final_span, *span,
                    // ))
                } else {
                    ConstantModifier::Final(*span)
                }
            }
            TokenKind::Final => {
                final_span = Some(*span);
                if let Some(private_span) = private_span {
                    todo!("tolerant mode")
                    // Err(error::final_and_private_modifiers_combined_for_constant(
                    //     *span,
                    //     private_span,
                    // ))
                } else {
                    ConstantModifier::Final(*span)
                }
            }
            _ => todo!("tolerant mode"), /* Err(error::modifier_cannot_be_used_for_constant(
                                             token.to_string(),
                                             *span,
                                         )) */
        })
        .collect::<Vec<ConstantModifier>>();

    ConstantModifierGroup { modifiers }
}

pub fn collect(state: &mut State) -> Vec<(Span, TokenKind)> {
    let mut collected: Vec<(Span, TokenKind)> = vec![];

    let collectable_tokens = vec![
        TokenKind::Private,
        TokenKind::Protected,
        TokenKind::Public,
        TokenKind::Final,
        TokenKind::Abstract,
        TokenKind::Static,
        TokenKind::Readonly,
    ];

    let mut current = state.stream.current().clone();
    let mut current_kind = current.kind;
    let mut current_span = current.span;

    while collectable_tokens.contains(&current_kind) {
        if let Some((span, _)) = collected.iter().find(|(_, kind)| kind == &current_kind) {
            todo!("tolerant mode");
            // return Err(error::multiple_modifiers(
            //     current_kind.to_string(),
            //     *span,
            //     current_span,
            // ));
        }

        // guard against multiple visibility modifiers, we don't care where these modifiers are used.
        if matches!(
            current_kind,
            TokenKind::Public | TokenKind::Protected | TokenKind::Private
        ) {
            if let Some((span, visibility)) = collected.iter().find(|(_, kind)| {
                matches!(
                    kind,
                    TokenKind::Public | TokenKind::Protected | TokenKind::Private
                )
            }) {
                todo!("tolerant mode")
                // state.record(error::multiple_visibility_modifiers(
                //     (visibility.to_string(), *span),
                //     (current_kind.to_string(), current_span),
                // ));
            }
        }

        collected.push((current_span, current_kind));

        state.stream.next();

        current = state.stream.current().clone();
        current_kind = current.kind;
        current_span = current.span;
    }

    collected
}
