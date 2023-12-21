use pxp_span::Span;

#[derive(Debug, Clone, Default)]
pub struct Location {
    pub file: String,
    pub span: Span,
}

impl Location {
    pub fn new(file: String, span: Span) -> Self {
        Self { file, span }
    }
}
