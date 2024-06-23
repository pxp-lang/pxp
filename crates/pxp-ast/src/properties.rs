use crate::{PropertyEntryKind, SimpleVariable};

impl PropertyEntryKind {
    pub fn variable(&self) -> &SimpleVariable {
        match self {
            PropertyEntryKind::Uninitialized { variable, .. } => variable,
            PropertyEntryKind::Initialized { variable, .. } => variable,
        }
    }

    pub fn is_initialized(&self) -> bool {
        match self {
            PropertyEntryKind::Uninitialized { .. } => false,
            PropertyEntryKind::Initialized { .. } => true,
        }
    }
}
