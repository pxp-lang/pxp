use crate::{PropertyEntry, SimpleVariable};

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
