use pxp_ast::{Node, ResolvedName};
use pxp_diagnostics::Severity;
use pxp_inference::TypeMap;
use pxp_span::Span;
use pxp_type::Type;

use crate::{AnalyserDiagnostic, Reporter};

pub struct AnalyserContext<'a> {
    reporter: &'a mut Reporter,
    types: TypeMap,
    file: usize,
}

impl<'a> AnalyserContext<'a> {
    pub fn new(reporter: &'a mut Reporter, types: TypeMap, file: usize) -> Self {
        Self { reporter, types, file }
    }

    pub fn report(&mut self, diagnostic: AnalyserDiagnostic, severity: Severity, span: Span) {
        self.reporter.report(self.file, diagnostic, severity, span);
    }

    pub fn get_type(&self, node: &Node) -> &Type<ResolvedName> {
        self.types.resolve(node.id)
    }
}
