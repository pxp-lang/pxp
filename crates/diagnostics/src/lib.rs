mod severity;

use pxp_span::Span;
pub use severity::*;

pub trait DiagnosticKind {
    fn get_code(&self) -> String;
    fn get_identifier(&self) -> String;
    fn get_message(&self) -> String;
    fn get_help(&self) -> Option<String> {
        None
    }
    fn get_labels(&self) -> Vec<DiagnosticLabel> {
        Vec::new()
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticLabel {
    pub style: DiagnosticLabelStyle,
    pub span: Span,
    pub message: String,
}

impl DiagnosticLabel {
    pub fn new(style: DiagnosticLabelStyle, span: Span, message: impl Into<String>) -> Self {
        Self {
            style,
            span,
            message: message.into(),
        }
    }

    pub fn primary(span: Span, message: impl Into<String>) -> Self {
        Self::new(DiagnosticLabelStyle::Primary, span, message)
    }

    pub fn secondary(span: Span, message: impl Into<String>) -> Self {
        Self::new(DiagnosticLabelStyle::Secondary, span, message)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticLabelStyle {
    Primary,
    Secondary,
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
