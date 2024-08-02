use crate::{
    InitializedPropertyEntry, PropertyEntryKind, SimpleVariable, UninitializedPropertyEntry,
};

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
