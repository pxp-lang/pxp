use dashmap::DashMap;
use pxp_diagnostics::Diagnostic;
use pxp_parser::ParserDiagnostic;
use tower_lsp::lsp_types::{TextDocumentContentChangeEvent, Url};

pub struct TextDocuments(pub DashMap<Url, Vec<u8>>);

impl TextDocuments {
    pub fn new() -> Self {
        TextDocuments(DashMap::new())
    }

    pub fn get(&self, uri: &Url) -> Option<Vec<u8>> {
        self.0.get(uri).map(|v| v.value().clone())
    }

    pub fn add(&self, uri: Url, content: Vec<u8>) {
        self.0.insert(uri, content);
    }

    pub fn update(&self, uri: &Url, changes: &[TextDocumentContentChangeEvent]) {
        if let Some(mut content) = self.0.get_mut(uri) {
            // We sync documents in full, so we can just replace the content.
            // In the future, it would be nice to switch over to incremental updates
            // to reduce the amount of data transferred.
            *content.value_mut() = changes[0].text.clone().into_bytes();
        }
    }
}

pub struct State {
    diagnostics: dashmap::DashMap<Url, Vec<Diagnostic<ParserDiagnostic>>>,
    pub(crate) text_documents: TextDocuments,
}

impl State {
    pub fn new() -> Self {
        State {
            diagnostics: dashmap::DashMap::new(),
            text_documents: TextDocuments::new(),
        }
    }

    pub fn add_diagnostic(&self, uri: Url, diagnostics: Vec<Diagnostic<ParserDiagnostic>>) {
        self.diagnostics.insert(uri, diagnostics);
    }

    pub fn remove_diagnostics_for_uri(&self, uri: &Url) {
        self.diagnostics.remove(uri);
    }
}