use crate::attributes::AttributeGroup;
use crate::modifiers::PropertyModifierGroup;

use crate::variables::SimpleVariable;
use crate::Expression;
use pxp_span::Span;
use pxp_type::Type;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Property {
    pub attributes: Vec<AttributeGroup>,

    pub modifiers: PropertyModifierGroup,
    pub r#type: Option<Type>,
    pub entries: Vec<PropertyEntry>,
    pub end: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct VariableProperty {
    pub attributes: Vec<AttributeGroup>,
    pub r#type: Option<Type>,
    pub entries: Vec<PropertyEntry>,
    pub end: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum PropertyEntry {
    Uninitialized {
        variable: SimpleVariable,
    },
    Initialized {
        variable: SimpleVariable,
        equals: Span,
        value: Expression,
    },
}

impl PropertyEntry {
    pub fn variable(&self) -> &SimpleVariable {
        match self {
            PropertyEntry::Uninitialized { variable } => variable,
            PropertyEntry::Initialized { variable, .. } => variable,
        }
    }

    pub fn is_initialized(&self) -> bool {
        match self {
            PropertyEntry::Uninitialized { .. } => false,
            PropertyEntry::Initialized { .. } => true,
        }
    }
}
