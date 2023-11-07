use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub enum Visibility {
    Public,
    Protected,
    Private,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum VisibilityModifier {
    Public(Span),
    Protected(Span),
    Private(Span),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum PromotedPropertyModifier {
    Public(Span),
    Protected(Span),
    Private(Span),
    Readonly(Span),
}

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

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[repr(transparent)]
pub struct PromotedPropertyModifierGroup {
    pub modifiers: Vec<PromotedPropertyModifier>,
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

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum PropertyModifier {
    Public(Span),
    Protected(Span),
    Private(Span),
    Static(Span),
    Readonly(Span),
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

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[repr(transparent)]
pub struct PropertyModifierGroup {
    pub modifiers: Vec<PropertyModifier>,
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

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum MethodModifier {
    Final(Span),
    Static(Span),
    Abstract(Span),
    Public(Span),
    Protected(Span),
    Private(Span),
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

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[repr(transparent)]
pub struct MethodModifierGroup {
    pub modifiers: Vec<MethodModifier>,
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

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum ClassModifier {
    Final(Span),
    Abstract(Span),
    Readonly(Span),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[repr(transparent)]
pub struct ClassModifierGroup {
    pub modifiers: Vec<ClassModifier>,
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

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum ConstantModifier {
    Final(Span),
    Public(Span),
    Protected(Span),
    Private(Span),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[repr(transparent)]
pub struct ConstantModifierGroup {
    pub modifiers: Vec<ConstantModifier>,
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
