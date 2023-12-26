use std::fmt::Display;

use pxp_span::Span;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum BackedEnumType {
    String(Span, Span), // `:` + `string`
    Int(Span, Span),    // `:` + `int`
    Invalid(Span),
}

impl BackedEnumType {
    pub fn is_valid(&self) -> bool {
        match self {
            Self::String(..) | Self::Int(..) => true,
            Self::Invalid(..) => false,
        }
    }
}

impl Display for BackedEnumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackedEnumType::String(..) => write!(f, "string"),
            BackedEnumType::Int(..) => write!(f, "int"),
            BackedEnumType::Invalid(..) => write!(f, "invalid"),
        }
    }
}

impl Default for BackedEnumType {
    fn default() -> Self {
        Self::Invalid(Span::default())
    }
}