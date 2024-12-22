use crate::{
    HookedProperty, InitializedPropertyEntry, Property, PropertyEntryKind, PropertyModifierGroup,
    SimpleProperty, SimpleVariable, UninitializedPropertyEntry,
};

impl Property {
    pub fn modifiers(&self) -> &PropertyModifierGroup {
        match self {
            Property::Simple(SimpleProperty { modifiers, .. }) => modifiers,
            Property::Hooked(HookedProperty { modifiers, .. }) => modifiers,
        }
    }

    pub fn is_public(&self) -> bool {
        self.modifiers().is_public()
    }

    pub fn is_protected(&self) -> bool {
        self.modifiers().is_protected()
    }

    pub fn is_private(&self) -> bool {
        self.modifiers().is_private()
    }

    pub fn is_static(&self) -> bool {
        self.modifiers().has_static()
    }

    pub fn is_readonly(&self) -> bool {
        self.modifiers().has_readonly()
    }
}

impl PropertyEntryKind {
    pub fn variable(&self) -> &SimpleVariable {
        match self {
            PropertyEntryKind::Uninitialized(UninitializedPropertyEntry { variable, .. }) => {
                variable
            }
            PropertyEntryKind::Initialized(InitializedPropertyEntry { variable, .. }) => variable,
        }
    }

    pub fn is_initialized(&self) -> bool {
        match self {
            PropertyEntryKind::Uninitialized { .. } => false,
            PropertyEntryKind::Initialized { .. } => true,
        }
    }
}
