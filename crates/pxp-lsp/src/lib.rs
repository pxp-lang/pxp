use std::error::Error;

mod client;
pub mod testing;
pub mod types;

pub use client::Client;
use lsp_server::{Connection, ExtractError, IoThreads, Message, Request, RequestId, Response};
use lsp_types::{notification::*, request::Request as _, *};
use request::{Completion, DocumentSymbolRequest, HoverRequest};
use serde_json::{from_value, to_value, Value};

pub type Result<T> = std::result::Result<T, Box<dyn Error + Sync + Send>>;

pub trait LanguageServer {
    fn initialize(&mut self, _: &Client, _: &InitializeParams) -> InitializeResult;

    fn initialized(&mut self, _: &Client) -> Result<()> {
        Ok(())
    }

    fn document_symbols(
        &mut self,
        _: &Client,
        _: &DocumentSymbolParams,
    ) -> Result<DocumentSymbolResponse> {
        Ok(DocumentSymbolResponse::Flat(Vec::new()))
    }

    fn hover(&mut self, _: &Client, _: &HoverParams) -> Result<Option<Hover>> {
        Ok(None)
    }

    fn completion(&mut self, _: &Client, _: &CompletionParams) -> Result<Vec<CompletionItem>> {
        Ok(Vec::new())
    }

    fn did_open(&mut self, _: &Client, _: &DidOpenTextDocumentParams) {}

    fn did_close(&mut self, _: &Client, _: &DidCloseTextDocumentParams) {}

    fn did_change(&mut self, _: &Client, _: &DidChangeTextDocumentParams) {}

    fn did_save(&mut self, _: &Client, _: &DidSaveTextDocumentParams) {}

    /// Manually handle a notification.
    ///
    /// Return `true` if the notification was handled, `false` otherwise.
    fn notification(&mut self, _: &Client, _method: &str, _params: &Value) -> Result<bool> {
        Ok(false)
    }
}

pub struct ServerManager<T: LanguageServer> {
    server: T,
}

impl<T: LanguageServer> ServerManager<T> {
    pub fn new(builder: impl FnOnce() -> T) -> Self {
        Self { server: builder() }
    }

    pub(crate) fn initialize(&mut self, connection: &Connection) -> Result<()> {
        let client = Client {
            connection: &connection,
        };

        let (id, params) = connection.initialize_start()?;
        let (id, initialize_params) = (id, from_value(params)?);

        // Get the server capabilities from the language server.
        let initialize_result = self.server.initialize(&client, &initialize_params);

        // Respond to the `initialize` request.
        connection.initialize_finish(id, to_value(initialize_result)?)?;

        // `initialize_finish()` takes care of the `initialized` message, so we can invoke the handler.
        self.server.initialized(&client)?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        let (connection, threads) = Connection::stdio();

        self.initialize(&connection)?;

        let client = Client {
            connection: &connection,
        };

        // Then we can start up the main loop.
        for msg in &connection.receiver {
            match msg {
                Message::Request(request) => {
                    if connection.handle_shutdown(&request)? {
                        return Ok(());
                    }

                    // Now we can map the request method to the appropriate handler.
                    match request.method.as_str() {
                        DocumentSymbolRequest::METHOD => {
                            let (id, params) = self.cast::<DocumentSymbolRequest>(request)?;
                            let response = self.server.document_symbols(&client, &params)?;

                            connection.sender.send(Message::Response(Response {
                                id,
                                result: Some(to_value(response)?),
                                error: None,
                            }))?;
                        }
                        HoverRequest::METHOD => {
                            let (id, params) = self.cast::<HoverRequest>(request)?;
                            let response = self.server.hover(&client, &params)?;

                            connection.sender.send(Message::Response(Response {
                                id,
                                result: Some(to_value(response)?),
                                error: None,
                            }))?;
                        }
                        Completion::METHOD => {
                            let (id, params) = self.cast::<Completion>(request)?;
                            let response = self.server.completion(&client, &params)?;

                            connection.sender.send(Message::Response(Response {
                                id,
                                result: Some(to_value(response)?),
                                error: None,
                            }))?;
                        }
                        _ => {}
                    }
                }
                Message::Response(_) => {}
                Message::Notification(notification) => {
                    if self.server.notification(
                        &client,
                        &notification.method,
                        &notification.params,
                    )? {
                        continue;
                    }

                    match notification.method.as_str() {
                        DidOpenTextDocument::METHOD => self
                            .server
                            .did_open(&client, &from_value(notification.params)?),
                        DidChangeTextDocument::METHOD => self
                            .server
                            .did_change(&client, &from_value(notification.params)?),
                        DidCloseTextDocument::METHOD => self
                            .server
                            .did_close(&client, &from_value(notification.params)?),
                        DidSaveTextDocument::METHOD => self
                            .server
                            .did_save(&client, &from_value(notification.params)?),
                        _ => {}
                    }
                }
            }
        }

        threads.join()?;

        Ok(())
    }

    fn cast<R>(
        &self,
        req: Request,
    ) -> std::result::Result<(RequestId, R::Params), ExtractError<Request>>
    where
        R: lsp_types::request::Request,
        R::Params: serde::de::DeserializeOwned,
    {
        req.extract(R::METHOD)
    }
}
