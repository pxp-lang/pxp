use std::fmt::Display;

use crate::BackedEnumType;

impl BackedEnumType {
    pub fn is_valid(&self) -> bool {
        match self {
            Self::String(..) | Self::Int(..) => true,
            Self::Invalid => false,
        }
    }
}

impl Display for BackedEnumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackedEnumType::String(..) => write!(f, "string"),
            BackedEnumType::Int(..) => write!(f, "int"),
            BackedEnumType::Invalid => write!(f, "invalid"),
        }
    }
}

impl Default for BackedEnumType {
    fn default() -> Self {
        Self::Invalid
    }
}
