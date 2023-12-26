use pxp_span::Span;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Location {
    pub file: String,
    pub span: Span,
}

impl Location {
    pub fn new(file: String, span: Span) -> Self {
        Self { file, span }
    }
}
