use std::collections::HashMap;

use pxp_diagnostics::{Diagnostic, DiagnosticKind, DiagnosticLabel, Severity};
use pxp_span::Span;

#[derive(Debug)]
pub struct Reporter {
    diagnostics: HashMap<usize, Vec<Diagnostic<AnalyserDiagnostic>>>,
}

#[derive(Debug)]
pub struct AnalyserDiagnostic {
    code: String,
    identifier: String,
    message: String,
    help: Option<String>,
    labels: Vec<DiagnosticLabel>,
}

impl AnalyserDiagnostic {
    pub fn new() -> Self {
        Self {
            code: String::new(),
            identifier: String::new(),
            message: String::new(),
            help: None,
            labels: Vec::new(),
        }
    }

    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = code.into();
        self
    }

    pub fn identifier(mut self, identifier: impl Into<String>) -> Self {
        self.identifier = identifier.into();
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn label(mut self, label: DiagnosticLabel) -> Self {
        self.labels.push(label);
        self
    }

    pub fn labels(mut self, labels: Vec<DiagnosticLabel>) -> Self {
        self.labels = labels;
        self
    }
}

impl DiagnosticKind for AnalyserDiagnostic {
    fn get_code(&self) -> String {
        self.code.clone()
    }

    fn get_identifier(&self) -> String {
        self.identifier.clone()
    }

    fn get_message(&self) -> String {
        self.message.clone()
    }

    fn get_help(&self) -> Option<String> {
        self.help.clone()
    }

    fn get_labels(&self) -> Vec<DiagnosticLabel> {
        self.labels.clone()
    }
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            diagnostics: HashMap::new(),
        }
    }

    pub fn report(
        &mut self,
        file: usize,
        diagnostic: AnalyserDiagnostic,
        severity: Severity,
        span: Span,
    ) {
        let diagnostics = self.diagnostics.entry(file).or_default();

        diagnostics.push(Diagnostic::new(diagnostic, severity, span));
    }

    pub fn all(&self) -> &HashMap<usize, Vec<Diagnostic<AnalyserDiagnostic>>> {
        &self.diagnostics
    }
}
