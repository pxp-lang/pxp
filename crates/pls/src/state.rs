use pxp_diagnostics::Diagnostic;
use pxp_parser::ParserDiagnostic;
use tower_lsp::lsp_types::Url;

pub struct State {
    diagnostics: dashmap::DashMap<Url, Vec<Diagnostic<ParserDiagnostic>>>
}

impl State {
    pub fn new() -> Self {
        State {
            diagnostics: dashmap::DashMap::new()
        }
    }

    pub fn add_diagnostic(&self, uri: Url, diagnostics: Vec<Diagnostic<ParserDiagnostic>>) {
        self.diagnostics.insert(uri, diagnostics);
    }

    pub fn remove_diagnostics_for_uri(&self, uri: &Url) {
        self.diagnostics.remove(uri);
    }
}