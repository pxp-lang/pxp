mod severity;

use std::fmt::Display;

use pxp_span::Span;
pub use severity::*;

pub trait DiagnosticKind {
    fn code(&self) -> &str;
    fn identifier(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct Diagnostic<K: Display + DiagnosticKind> {
    pub kind: K,
    pub severity: Severity,
    pub span: Span,
}

impl<K: Display + DiagnosticKind> Diagnostic<K> {
    pub fn new(kind: K, severity: Severity, span: Span) -> Self {
        Self {
            kind,
            severity,
            span,
        }
    }
}

impl<K: Display + DiagnosticKind> Display for Diagnostic<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.severity, self.kind)
    }
}
