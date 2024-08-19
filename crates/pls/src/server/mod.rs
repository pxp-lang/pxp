use std::error::Error;

use lsp_server::{Connection, ExtractError, Message, Request, RequestId};
use lsp_textdocument::TextDocuments;
use lsp_types::{*, notification::*, request::*};
use serde_json::{from_value, to_value, Value};

pub trait LanguageServer {
    fn initialize(&mut self, _: &InitializeParams) -> InitializeResult;

    fn initialized(&mut self) {
        
    }

    fn did_open(&mut self, _: &DidOpenTextDocumentParams) {

    }

    fn did_close(&mut self, _: &DidCloseTextDocumentParams) {

    }

    fn did_change(&mut self, _: &DidChangeTextDocumentParams) {

    }

    fn did_save(&mut self, _: &DidSaveTextDocumentParams) {

    }

    /// Manually handle a notification.
    /// 
    /// Return `true` if the notification was handled, `false` otherwise.
    fn notification(&mut self, _method: &str, _params: &Value) -> bool {
        false
    }
}

pub struct ServerManager<T: LanguageServer> {
    server: T
}

impl<T: LanguageServer> ServerManager<T> {
    pub fn new(builder: impl FnOnce() -> T) -> Self {
        Self { server: builder() }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error + Sync + Send>> {
        // FIXME: Make this configurable, so we can use stdio or tcp.
        let (connection, threads) = Connection::stdio();

        // Wait for an `initialize` request from the client.
        let (id, initialize_params): (RequestId, InitializeParams) = match connection.initialize_start()? {
            (id, params) => (id, from_value(params)?),
        };

        // Get the server capabilities from the language server.
        let initialize_result = self.server.initialize(&initialize_params);

        // Respond to the `initialize` request.
        connection.initialize_finish(id, to_value(initialize_result)?)?;
        
        // `initialize_finish()` takes care of the `initialized` message, so we can invoke the handler.
        self.server.initialized();

        // Then we can start up the main loop.
        for msg in &connection.receiver {
            match msg {
                Message::Request(request) => {
                    if connection.handle_shutdown(&request)? {
                        return Ok(());
                    }

                    // Now we can map the request method to the appropriate handler.
                    match request.method.as_str() {
                        _ => {},
                    }
                },
                Message::Response(response) => {

                },
                Message::Notification(notification) => {
                    if self.server.notification(&notification.method, &notification.params) {
                        continue;
                    }

                    match notification.method.as_str() {
                        DidOpenTextDocument::METHOD => self.server.did_open(&from_value(notification.params)?),
                        DidChangeTextDocument::METHOD => self.server.did_change(&from_value(notification.params)?),
                        DidCloseTextDocument::METHOD => self.server.did_close(&from_value(notification.params)?),
                        DidSaveTextDocument::METHOD => self.server.did_save(&from_value(notification.params)?),
                        _ => {},
                    }
                },
            }
        }

        threads.join()?;

        Ok(())
    }

    fn cast<R>(&self, req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
    where
        R: lsp_types::request::Request,
        R::Params: serde::de::DeserializeOwned,
    {
        req.extract(R::METHOD)
    }
}