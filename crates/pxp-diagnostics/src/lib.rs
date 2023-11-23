mod kind;
mod severity;

pub use kind::*;
use pxp_span::Span;
pub use severity::*;

#[derive(Debug)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub severity: Severity,
    pub span: Span,
}

impl Diagnostic {
    pub fn new(kind: DiagnosticKind, severity: Severity, span: Span) -> Self {
        Self {
            kind,
            severity,
            span,
        }
    }
}
