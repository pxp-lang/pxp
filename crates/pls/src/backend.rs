use std::collections::HashMap;

use lsp_textdocument::TextDocuments;
use lsp_types::{notification::{DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, DidSaveTextDocument, Notification}, CompletionItem, CompletionParams, Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams, DocumentSymbolParams, DocumentSymbolResponse, Hover, HoverParams, InitializeParams, InitializeResult, MessageType, Position, Range, ServerInfo, Uri};
use pxp_diagnostics::{Diagnostic as InternalDiagnostic, Severity};
use pxp_index::{Index, Indexer};
use pxp_parser::{parse, ParserDiagnostic};
use pxp_span::{Spanned};
use serde_json::{from_value, Value};

use crate::{capabilities::get_server_capabilities, server::{Client, LanguageServer, Result}};

pub struct Backend {
    pub documents: TextDocuments,
    pub diagnostics: HashMap<Uri, Vec<InternalDiagnostic<ParserDiagnostic>>>,
    pub index: Index,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            documents: TextDocuments::new(),
            diagnostics: HashMap::new(),
            index: Index::new(),
        }
    }

    fn index_document(&mut self, uri: &Uri) {
        if let Some(document) = self.documents.get_document(uri) {
            let content = document.get_content(None).as_bytes();
            let parse_result = parse(&content);

            // FIXME: Remove this clone, since it will be expensive.
            let mut indexer = Indexer::for_index(self.index.clone());
            indexer.index(&parse_result.ast);
            
            // FIXME: Remove this clone, since it will be expensive.
            self.index = indexer.get_index().clone();
        }
    }

    fn send_diagnostics(&mut self, client: &Client, uri: &Uri) -> Result<()> {
        client.log_message(
            MessageType::INFO,
            format!("Sending diagnostics for [`{}`].", uri.as_str()),
        )?;

        if let Some(document) = self.documents.get_document(uri) {
            let content = document.get_content(None).as_bytes();
            let diagnostics = parse(&content).diagnostics;

            self.diagnostics.insert(uri.clone(), diagnostics.clone());
            self.publish_diagnostics(client, uri, &diagnostics, document.version(), content)?;
        }

        Ok(())
    }

    fn publish_diagnostics(&self, client: &Client, uri: &Uri, parser_diagnostics: &[InternalDiagnostic<ParserDiagnostic>], version: i32, content: &[u8]) -> Result<()> {
        client.log_message(
            MessageType::INFO,
            format!("Publishing diagnostics for [`{}`].", uri.as_str()),
        )?;

        let mut diagnostics = Vec::new();

        parser_diagnostics.iter().for_each(|d| {
            let severity = match d.severity {
                Severity::Hint => DiagnosticSeverity::HINT,
                Severity::Information => DiagnosticSeverity::INFORMATION,
                Severity::Warning => DiagnosticSeverity::WARNING,
                Severity::Error => DiagnosticSeverity::ERROR,
            };

            let (start_line, start_col, end_line, end_col) = (
                d.span.start_line(content),
                d.span.start_column(content),
                d.span.end_line(content),
                d.span.end_column(content),
            );

            let message = d.to_string();

            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: start_line as u32,
                        character: start_col as u32,
                    },
                    end: Position {
                        line: end_line as u32,
                        character: end_col as u32,
                    },
                },
                severity: Some(severity),
                message,
                ..Default::default()
            });
        });

        client.publish_diagnostics(uri, diagnostics, version)?;

        Ok(())
    }
}

impl LanguageServer for Backend {
    fn initialize(&mut self, _: &Client, _: &InitializeParams) -> InitializeResult {
        InitializeResult {
            capabilities: get_server_capabilities(),
            server_info: Some(ServerInfo {
                name: "PLS (PHP Language Server)".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        }
    }

    fn initialized(&mut self, client: &Client) -> Result<()> {
        // FIXME: Index workspace here.
        client.log_message(MessageType::INFO, "Language server initialized.".to_string())
    }

    fn document_symbols(&mut self, client: &Client, params: &DocumentSymbolParams) -> Result<DocumentSymbolResponse> {
        client.log_message(MessageType::INFO, format!("Generating document symbols for [`{}`].", &params.text_document.uri.to_string()))?;

        let symbols = self.get_document_symbols(&params.text_document.uri)?;

        Ok(DocumentSymbolResponse::Nested(symbols))
    }

    fn hover(&mut self, client: &Client, params: &HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;

        client.log_message(MessageType::INFO, format!("Generating hover information for [`{}`].", **uri))?;

        Ok(self.generate_hover(uri, &params.text_document_position_params.position))
    }

    fn completion(&mut self, _: &Client, params: &CompletionParams) -> Result<Vec<CompletionItem>> {
        self.get_completion_items(&params.text_document_position.text_document.uri, params.text_document_position.position)
    }

    fn notification(&mut self, client: &Client, method: &str, params: &Value) -> Result<bool> {
        if self.documents.listen(method, params) {
            client.log_message(
                MessageType::INFO,
                format!("Accepted document change notification [`{method}`].")
            )?;

            let uri = match method {
                DidOpenTextDocument::METHOD => {
                    let params: DidOpenTextDocumentParams = from_value(params.clone())?;
                    params.text_document.uri
                },
                DidChangeTextDocument::METHOD => {
                    let params: DidChangeTextDocumentParams = from_value(params.clone())?;
                    params.text_document.uri
                },
                DidSaveTextDocument::METHOD => {
                    let params: DidSaveTextDocumentParams = from_value(params.clone())?;
                    params.text_document.uri
                },
                DidCloseTextDocument::METHOD => {
                    let params: DidCloseTextDocumentParams = from_value(params.clone())?;

                    self.diagnostics.remove(&params.text_document.uri);

                    return Ok(true)
                },
                _ => return Ok(true),
            };

            // self.send_diagnostics(client, &uri)?;
            self.index_document(&uri);

            return Ok(true);
        }

        Ok(false)
    }
}