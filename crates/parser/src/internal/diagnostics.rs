use pxp_diagnostics::{Diagnostic, Severity};
use pxp_span::Span;

use crate::{Parser, ParserDiagnostic};

impl<'a> Parser<'a> {
    pub(crate) fn diagnostic(
        &mut self,
        diagnostic: ParserDiagnostic,
        severity: Severity,
        span: Span,
    ) {
        self.diagnostics
            .push(Diagnostic::new(diagnostic, severity, span));
    }
}
