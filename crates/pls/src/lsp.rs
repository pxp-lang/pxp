use std::sync::Mutex;

use crate::capabilities::get_server_capabilities;
use crate::state::State;
use pxp_diagnostics::{Diagnostic, Severity};
use pxp_parser::ParseResult;
use pxp_parser::ParserDiagnostic;
use pxp_span::Span;
use pxp_span::Spanned;
use tower_lsp::async_trait;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::DiagnosticSeverity;
use tower_lsp::lsp_types::*;
use tower_lsp::Client;
use tower_lsp::LanguageServer;

pub(crate) struct Backend {
    pub(crate) client: Client,
    state: Mutex<State>
}

impl Backend {
    pub(crate) fn new(client: Client) -> Backend {
        Backend {
            client,
            state: Mutex::new(State::new())
        }
    }

    async fn send_diagnostics(&self, uri: Url, content: String) {
        let diagnostics = self.calculate_diagnostics(&uri, &content).await;
        self.publish_diagnostics_to_client(uri, &content, &diagnostics).await;
    }

    async fn add_document(&self, uri: Url, content: String) {
        if let Ok(state) = self.state.lock() {
            state.text_documents.add(uri, content.as_bytes().to_vec());
        }
    }

    async fn update_document(&self, uri: &Url, changes: &[TextDocumentContentChangeEvent]) {        
        if let Ok(state) = self.state.lock() {
            state.text_documents.update(uri, changes);
        }
    }

    async fn calculate_diagnostics(&self, uri: &Url, content: &String) -> Vec<Diagnostic<ParserDiagnostic>>
    {
        let parse_result: ParseResult = pxp_parser::parse(&content);

        if let Ok(state) = self.state.lock() {
            state.add_diagnostic(uri.clone(), parse_result.diagnostics.clone());
        }

        parse_result.diagnostics
    }

    async fn publish_diagnostics_to_client(
        &self,
        uri: Url,
        text: &str,
        parser_diagnostics: &[Diagnostic<ParserDiagnostic>],
    ) {
        self.client.log_message(MessageType::INFO, "Publishing diagnostics".to_string()).await;
        
        let mut diagnostics = Vec::new();

        parser_diagnostics.iter().for_each(|d| {
            let severity = match d.severity {
                Severity::Hint        => DiagnosticSeverity::HINT,
                Severity::Information => DiagnosticSeverity::INFORMATION,
                Severity::Warning     => DiagnosticSeverity::WARNING,
                Severity::Error       => DiagnosticSeverity::ERROR,
            };
            
            let range = self.calculate_line_and_character(text, d.span, d.span);

            let message = d.to_string();

            diagnostics.push(lsp_types::Diagnostic {
                range,
                severity: Some(severity),
                code: None,
                code_description: None,
                source: None,
                message,
                related_information: None,
                tags: None,
                data: None,
            });
        });

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    fn calculate_line_and_character(&self, text: &str, start: Span, end: Span) -> Range {
        let (start_line, start_col) = (start.start_line(text.as_bytes()), start.start_column(text.as_bytes()));
        let (end_line, end_col) = (end.end_line(text.as_bytes()), end.end_column(text.as_bytes()));

        Range {
            start: Position {
                line: start_line as u32,
                character: start_col as u32,
            },
            end: Position {
                line: end_line as u32,
                character: end_col as u32
            }
        }
    }
}

#[async_trait]
impl LanguageServer for Backend {
    // Lifecycle Messages
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        return Ok(InitializeResult {
            capabilities: get_server_capabilities(_params),
            server_info: Some(ServerInfo {
                name: "PLS (PHP Language Server)".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            offset_encoding: None,
        });
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(
                MessageType::INFO,
                "PLS initialized.".to_string(),
            )
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;

        self.add_document(uri.clone(), content.clone()).await;
        self.send_diagnostics(uri, content).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.content_changes[0].text.clone();
        let changes = params.content_changes;

        self.update_document(&uri, &changes).await;
        self.send_diagnostics(uri, content).await;
    }

    async fn did_close(&self, _params: DidCloseTextDocumentParams) {
        let uri = _params.text_document.uri;

        self.client
            .log_message(MessageType::INFO, format!("Closed document: {}", uri))
            .await;

        if let Ok(state) = self.state.lock() {
            state.remove_diagnostics_for_uri(&uri);
        }
    }

    async fn did_save(&self, _params: DidSaveTextDocumentParams) {
        let uri = _params.text_document.uri;
        let content = _params.text;

        if let Some(c) = content {
            self.send_diagnostics(uri, c).await;
        }
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        Ok(self.calculate_hovers(&uri, position).await)
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
