use pxp_span::Span;
use pxp_syntax::visibility::Visibility;

use crate::{
    ClassModifier, ClassModifierGroup, ConstantModifier, ConstantModifierGroup, MethodModifier,
    MethodModifierGroup, PromotedPropertyModifier, PromotedPropertyModifierGroup, PropertyModifier,
    PropertyModifierGroup,
};

impl PromotedPropertyModifier {
    pub fn span(&self) -> Span {
        match self {
            PromotedPropertyModifier::Public(span) => *span,
            PromotedPropertyModifier::Protected(span) => *span,
            PromotedPropertyModifier::Private(span) => *span,
            PromotedPropertyModifier::Readonly(span) => *span,
        }
    }
}

impl std::fmt::Display for PromotedPropertyModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromotedPropertyModifier::Public(_) => write!(f, "public"),
            PromotedPropertyModifier::Protected(_) => write!(f, "protected"),
            PromotedPropertyModifier::Private(_) => write!(f, "private"),
            PromotedPropertyModifier::Readonly(_) => write!(f, "readonly"),
        }
    }
}

impl PromotedPropertyModifierGroup {
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    pub fn get_readonly(&self) -> Option<&PromotedPropertyModifier> {
        self.modifiers
            .iter()
            .find(|modifier| matches!(modifier, PromotedPropertyModifier::Readonly { .. }))
    }

    pub fn has_readonly(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, PromotedPropertyModifier::Readonly { .. }))
    }

    pub fn visibility(&self) -> Visibility {
        self.modifiers
            .iter()
            .find_map(|modifier| match modifier {
                PromotedPropertyModifier::Protected { .. } => Some(Visibility::Protected),
                PromotedPropertyModifier::Private { .. } => Some(Visibility::Private),
                PromotedPropertyModifier::Public { .. } => Some(Visibility::Public),
                _ => None,
            })
            .unwrap_or(Visibility::Public)
    }
}

impl PropertyModifier {
    pub fn span(&self) -> Span {
        match self {
            PropertyModifier::Public(span) => *span,
            PropertyModifier::Protected(span) => *span,
            PropertyModifier::Private(span) => *span,
            PropertyModifier::Static(span) => *span,
            PropertyModifier::Readonly(span) => *span,
        }
    }
}

impl PropertyModifierGroup {
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    pub fn get_readonly(&self) -> Option<&PropertyModifier> {
        self.modifiers
            .iter()
            .find(|modifier| matches!(modifier, PropertyModifier::Readonly { .. }))
    }

    pub fn get_static(&self) -> Option<&PropertyModifier> {
        self.modifiers
            .iter()
            .find(|modifier| matches!(modifier, PropertyModifier::Static { .. }))
    }

    pub fn has_readonly(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, PropertyModifier::Readonly { .. }))
    }

    pub fn has_static(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, PropertyModifier::Static { .. }))
    }

    pub fn visibility(&self) -> Visibility {
        self.modifiers
            .iter()
            .find_map(|modifier| match modifier {
                PropertyModifier::Protected { .. } => Some(Visibility::Protected),
                PropertyModifier::Private { .. } => Some(Visibility::Private),
                PropertyModifier::Public { .. } => Some(Visibility::Public),
                _ => None,
            })
            .unwrap_or(Visibility::Public)
    }
}

impl MethodModifier {
    pub fn span(&self) -> Span {
        match self {
            MethodModifier::Final(span) => *span,
            MethodModifier::Static(span) => *span,
            MethodModifier::Abstract(span) => *span,
            MethodModifier::Public(span) => *span,
            MethodModifier::Protected(span) => *span,
            MethodModifier::Private(span) => *span,
        }
    }
}

impl MethodModifierGroup {
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    pub fn has_final(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, MethodModifier::Final { .. }))
    }

    pub fn has_static(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, MethodModifier::Static { .. }))
    }

    pub fn has_abstract(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, MethodModifier::Abstract { .. }))
    }

    pub fn get_abstract(&self) -> Option<&MethodModifier> {
        self.modifiers
            .iter()
            .find(|modifier| matches!(modifier, MethodModifier::Abstract { .. }))
    }

    pub fn visibility(&self) -> Visibility {
        self.modifiers
            .iter()
            .find_map(|modifier| match modifier {
                MethodModifier::Protected { .. } => Some(Visibility::Protected),
                MethodModifier::Private { .. } => Some(Visibility::Private),
                MethodModifier::Public { .. } => Some(Visibility::Public),
                _ => None,
            })
            .unwrap_or(Visibility::Public)
    }
}

impl ClassModifierGroup {
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    pub fn has_final(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, ClassModifier::Final { .. }))
    }

    pub fn has_readonly(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, ClassModifier::Readonly { .. }))
    }

    pub fn has_abstract(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, ClassModifier::Abstract { .. }))
    }
}

impl ConstantModifierGroup {
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    pub fn has_final(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, ConstantModifier::Final { .. }))
    }

    pub fn has_private(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, ConstantModifier::Private { .. }))
    }

    pub fn visibility(&self) -> Visibility {
        self.modifiers
            .iter()
            .find_map(|modifier| match modifier {
                ConstantModifier::Protected { .. } => Some(Visibility::Protected),
                ConstantModifier::Private { .. } => Some(Visibility::Private),
                ConstantModifier::Public { .. } => Some(Visibility::Public),
                _ => None,
            })
            .unwrap_or(Visibility::Public)
    }
}
