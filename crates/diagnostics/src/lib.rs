mod severity;

use pxp_span::Span;
pub use severity::*;

pub trait DiagnosticKind {
    fn code(&self) -> String;
    fn identifier(&self) -> String;
    fn message(&self) -> String;
    fn help(&self) -> Option<String> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic<K: DiagnosticKind> {
    pub kind: K,
    pub severity: Severity,
    pub span: Span,
}

impl<K: DiagnosticKind> Diagnostic<K> {
    pub fn new(kind: K, severity: Severity, span: Span) -> Self {
        Self {
            kind,
            severity,
            span,
        }
    }
}
