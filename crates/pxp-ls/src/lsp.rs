use std::sync::Mutex;

use crate::capabilities::get_server_capabilities;
use pxp_diagnostics::{Diagnostic, Severity};
use pxp_parser::ParseResult;
use pxp_parser::ParserDiagnostic;
use pxp_symbol::SymbolTable;
use tower_lsp::async_trait;
use tower_lsp::jsonrpc::Error;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::DiagnosticSeverity;
use tower_lsp::lsp_types::*;
use tower_lsp::Client;
use tower_lsp::LanguageServer;

struct State {
    diagnostics: dashmap::DashMap<Url, Vec<Diagnostic<ParserDiagnostic>>>
}

pub(crate) struct Backend {
    pub(crate) client: Client,
    state: Mutex<State>
}

impl Backend {

    pub(crate) fn new(client: Client) -> Backend {
        Backend {
            client,
            state: Mutex::new(State {
                diagnostics: dashmap::DashMap::new()
            })
        }
    }

    async fn send_diagnostics(&self, uri: Url, content: String) {
        let diagnostics = self.calculate_diagnostics(&uri, &content).await;
        self.publish_diagnostics_to_client(uri, &content, &diagnostics).await;
    }

    async fn calculate_diagnostics(&self, uri: &Url, content: &String) -> Vec<Diagnostic<ParserDiagnostic>>
    {
        let mut table = SymbolTable::new();
        let parse_result: ParseResult = pxp_parser::parse(&content, &mut table);

        match self.state.lock() {
            Ok(state) => {
                state.diagnostics.insert(uri.clone(), parse_result.diagnostics.clone());
            },
            Err(_) => {}
        };

        parse_result.diagnostics
    }

    async fn publish_diagnostics_to_client(
        &self,
        uri: Url,
        text: &String,
        parser_diagnostics: &Vec<Diagnostic<ParserDiagnostic>>,
    ) {
        let mut diagnostics = Vec::new();

        parser_diagnostics.into_iter().for_each(|d| {
            let severity = match d.severity {
                Severity::Hint        => DiagnosticSeverity::HINT,
                Severity::Information => DiagnosticSeverity::INFORMATION,
                Severity::Warning     => DiagnosticSeverity::WARNING,
                Severity::Error       => DiagnosticSeverity::ERROR,
            };
            
            let range = self.calculate_line_and_character(&text, d.span.start as u32, d.span.end as u32);

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

    fn calculate_line_and_character(&self, text: &str, start: u32, end: u32) -> Range {
        let lines = text.split('\n').collect::<Vec<&str>>();
        let mut start_line: u32 = 0;
        let mut start_char: u32 = 0;
        let mut end_line: u32 = 0;
        let mut end_char: u32 = 0;

        let mut offset: u32 = 0;

        for (i, line) in lines.iter().enumerate() {
            let line_len = line.len() as u32;
            if offset + line_len >= start {
                start_line = i as u32;
                start_char = start - offset;
            }

            if offset + line_len >= end {
                end_line = i as u32;
                end_char = end - offset;
                break;
            }

            offset += line_len + 1;
        }

        Range {
            start: Position {
                line: start_line,
                character: start_char,
            },
            end: Position {
                line: end_line,
                character: end_char
            }
        }

    }
}

#[async_trait]
impl LanguageServer for Backend {
    // Lifecycle Messages
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult, Error> {
        return Ok(InitializeResult {
            capabilities: get_server_capabilities(_params),
            server_info: None,
            offset_encoding: None,
        });
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(
                MessageType::INFO,
                "PXP Language Server Initialized".to_string(),
            )
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;
        self.send_diagnostics(uri, content).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.content_changes[0].text.clone();
        self.send_diagnostics(uri, content).await;
    }

    async fn did_close(&self, _params: DidCloseTextDocumentParams) {
        let uri = _params.text_document.uri;
        self.client
            .log_message(MessageType::INFO, format!("Closed document: {}", uri))
            .await;

        match self.state.lock() {
            Ok(state) => {
                state.diagnostics.remove(&uri);
            },
            Err(_) => {}
        }
    }

    async fn did_save(&self, _params: DidSaveTextDocumentParams) {
        let uri = _params.text_document.uri;
        let content = _params.text;

        match content {
            Some(c) => self.send_diagnostics(uri, c).await,
            None => {}
        };
    }

    async fn shutdown(&self) -> Result<(), Error> {
        Ok(())
    }
}
