use crate::lexer::token::Span;
use crate::lexer::token::TokenKind;
use crate::parser::ast::modifiers::ClassModifier;
use crate::parser::ast::modifiers::ClassModifierGroup;
use crate::parser::ast::modifiers::ConstantModifier;
use crate::parser::ast::modifiers::ConstantModifierGroup;
use crate::parser::ast::modifiers::MethodModifier;
use crate::parser::ast::modifiers::MethodModifierGroup;
use crate::parser::ast::modifiers::PromotedPropertyModifier;
use crate::parser::ast::modifiers::PromotedPropertyModifierGroup;
use crate::parser::ast::modifiers::PropertyModifier;
use crate::parser::ast::modifiers::PropertyModifierGroup;
use crate::parser::error;
use crate::parser::error::ParseResult;
use crate::parser::state::State;

#[inline(always)]
pub fn class_group(input: Vec<(Span, TokenKind)>) -> ParseResult<ClassModifierGroup> {
    let mut final_span = None;
    let mut abstract_span = None;

    let modifiers = input
        .iter()
        .map(|(span, token)| match token {
            TokenKind::Readonly => Ok(ClassModifier::Readonly(*span)),
            TokenKind::Final => {
                final_span = Some(*span);
                if let Some(abstract_span) = abstract_span {
                    Err(error::final_and_abstract_modifiers_combined_for_class(
                        *span,
                        abstract_span,
                    ))
                } else {
                    Ok(ClassModifier::Final(*span))
                }
            }
            TokenKind::Abstract => {
                abstract_span = Some(*span);
                if let Some(final_span) = final_span {
                    Err(error::final_and_abstract_modifiers_combined_for_class(
                        final_span, *span,
                    ))
                } else {
                    Ok(ClassModifier::Abstract(*span))
                }
            }
            _ => Err(error::modifier_cannot_be_used_for_class(
                token.to_string(),
                *span,
            )),
        })
        .collect::<ParseResult<Vec<ClassModifier>>>()?;

    Ok(ClassModifierGroup { modifiers })
}

#[inline(always)]
pub fn method_group(input: Vec<(Span, TokenKind)>) -> ParseResult<MethodModifierGroup> {
    let mut final_span = None;
    let mut abstract_span = None;

    let modifiers = input
        .iter()
        .map(|(span, token)| match token {
            TokenKind::Final => {
                final_span = Some(*span);
                if let Some(abstract_span) = abstract_span {
                    Err(
                        error::final_and_abstract_modifiers_combined_for_class_member(
                            *span,
                            abstract_span,
                        ),
                    )
                } else {
                    Ok(MethodModifier::Final(*span))
                }
            }
            TokenKind::Abstract => {
                abstract_span = Some(*span);
                if let Some(final_span) = final_span {
                    Err(
                        error::final_and_abstract_modifiers_combined_for_class_member(
                            final_span, *span,
                        ),
                    )
                } else {
                    Ok(MethodModifier::Abstract(*span))
                }
            }
            TokenKind::Private => Ok(MethodModifier::Private(*span)),
            TokenKind::Protected => Ok(MethodModifier::Protected(*span)),
            TokenKind::Public => Ok(MethodModifier::Public(*span)),
            TokenKind::Static => Ok(MethodModifier::Static(*span)),
            _ => Err(error::modifier_cannot_be_used_for_class_method(
                token.to_string(),
                *span,
            )),
        })
        .collect::<ParseResult<Vec<MethodModifier>>>()?;

    Ok(MethodModifierGroup { modifiers })
}

#[inline(always)]
pub fn interface_method_group(input: Vec<(Span, TokenKind)>) -> ParseResult<MethodModifierGroup> {
    let modifiers = input
        .iter()
        .map(|(span, token)| match token {
            TokenKind::Public => Ok(MethodModifier::Public(*span)),
            TokenKind::Static => Ok(MethodModifier::Static(*span)),
            _ => Err(error::modifier_cannot_be_used_for_interface_method(
                token.to_string(),
                *span,
            )),
        })
        .collect::<ParseResult<Vec<MethodModifier>>>()?;

    Ok(MethodModifierGroup { modifiers })
}

#[inline(always)]
pub fn enum_method_group(input: Vec<(Span, TokenKind)>) -> ParseResult<MethodModifierGroup> {
    let modifiers = input
        .iter()
        .map(|(span, token)| match token {
            TokenKind::Final => Ok(MethodModifier::Final(*span)),
            TokenKind::Private => Ok(MethodModifier::Private(*span)),
            TokenKind::Protected => Ok(MethodModifier::Protected(*span)),
            TokenKind::Public => Ok(MethodModifier::Public(*span)),
            TokenKind::Static => Ok(MethodModifier::Static(*span)),
            _ => Err(error::modifier_cannot_be_used_for_enum_method(
                token.to_string(),
                *span,
            )),
        })
        .collect::<ParseResult<Vec<MethodModifier>>>()?;

    Ok(MethodModifierGroup { modifiers })
}

#[inline(always)]
pub fn property_group(input: Vec<(Span, TokenKind)>) -> ParseResult<PropertyModifierGroup> {
    let modifiers = input
        .iter()
        .map(|(span, token)| match token {
            TokenKind::Readonly => Ok(PropertyModifier::Readonly(*span)),
            TokenKind::Static => Ok(PropertyModifier::Static(*span)),
            TokenKind::Public => Ok(PropertyModifier::Public(*span)),
            TokenKind::Protected => Ok(PropertyModifier::Protected(*span)),
            TokenKind::Private => Ok(PropertyModifier::Private(*span)),
            _ => Err(error::modifier_cannot_be_used_for_property(
                token.to_string(),
                *span,
            )),
        })
        .collect::<ParseResult<Vec<PropertyModifier>>>()?;

    Ok(PropertyModifierGroup { modifiers })
}

#[inline(always)]
pub fn promoted_property_group(
    input: Vec<(Span, TokenKind)>,
) -> ParseResult<PromotedPropertyModifierGroup> {
    let modifiers = input
        .iter()
        .map(|(span, token)| match token {
            TokenKind::Readonly => Ok(PromotedPropertyModifier::Readonly(*span)),
            TokenKind::Private => Ok(PromotedPropertyModifier::Private(*span)),
            TokenKind::Protected => Ok(PromotedPropertyModifier::Protected(*span)),
            TokenKind::Public => Ok(PromotedPropertyModifier::Public(*span)),
            _ => Err(error::modifier_cannot_be_used_for_promoted_property(
                token.to_string(),
                *span,
            )),
        })
        .collect::<ParseResult<Vec<PromotedPropertyModifier>>>()?;

    Ok(PromotedPropertyModifierGroup { modifiers })
}

pub fn constant_group(input: Vec<(Span, TokenKind)>) -> ParseResult<ConstantModifierGroup> {
    let mut final_span = None;
    let mut private_span = None;

    let modifiers = input
        .iter()
        .map(|(span, token)| match token {
            TokenKind::Protected => Ok(ConstantModifier::Protected(*span)),
            TokenKind::Public => Ok(ConstantModifier::Public(*span)),
            TokenKind::Private => {
                private_span = Some(*span);
                if let Some(final_span) = final_span {
                    Err(error::final_and_private_modifiers_combined_for_constant(
                        final_span, *span,
                    ))
                } else {
                    Ok(ConstantModifier::Final(*span))
                }
            }
            TokenKind::Final => {
                final_span = Some(*span);
                if let Some(private_span) = private_span {
                    Err(error::final_and_private_modifiers_combined_for_constant(
                        *span,
                        private_span,
                    ))
                } else {
                    Ok(ConstantModifier::Final(*span))
                }
            }
            _ => Err(error::modifier_cannot_be_used_for_constant(
                token.to_string(),
                *span,
            )),
        })
        .collect::<ParseResult<Vec<ConstantModifier>>>()?;

    Ok(ConstantModifierGroup { modifiers })
}

pub fn interface_constant_group(
    input: Vec<(Span, TokenKind)>,
) -> ParseResult<ConstantModifierGroup> {
    let modifiers = input
        .iter()
        .map(|(span, token)| match token {
            TokenKind::Public => Ok(ConstantModifier::Public(*span)),
            TokenKind::Final => Ok(ConstantModifier::Final(*span)),
            _ => Err(error::modifier_cannot_be_used_for_interface_constant(
                token.to_string(),
                *span,
            )),
        })
        .collect::<ParseResult<Vec<ConstantModifier>>>()?;

    Ok(ConstantModifierGroup { modifiers })
}

pub fn collect(state: &mut State) -> ParseResult<Vec<(Span, TokenKind)>> {
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
            return Err(error::multiple_modifiers(
                current_kind.to_string(),
                *span,
                current_span,
            ));
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
                state.record(error::multiple_visibility_modifiers(
                    (visibility.to_string(), *span),
                    (current_kind.to_string(), current_span),
                ));
            }
        }

        collected.push((current_span, current_kind));

        state.stream.next();

        current = state.stream.current().clone();
        current_kind = current.kind;
        current_span = current.span;
    }

    Ok(collected)
}
