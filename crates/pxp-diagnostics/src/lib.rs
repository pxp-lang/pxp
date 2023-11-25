mod kind;
mod severity;

use std::fmt::Display;

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

impl Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} on line {}\n", self.severity, self.kind, self.span.start.line)
    }
}