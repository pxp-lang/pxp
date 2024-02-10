mod severity;

use std::fmt::Display;

use pxp_span::Span;
pub use severity::*;

#[derive(Debug)]
pub struct Diagnostic<K: Display> {
    pub kind: K,
    pub severity: Severity,
    pub span: Span,
}

impl<K: Display> Diagnostic<K> {
    pub fn new(kind: K, severity: Severity, span: Span) -> Self {
        Self {
            kind,
            severity,
            span,
        }
    }
}

impl<K: Display> Display for Diagnostic<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} on line {}",
            self.severity, self.kind, self.span.start.line
        )
    }
}
