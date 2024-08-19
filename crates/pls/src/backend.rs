use lsp_types::{InitializeParams, InitializeResult, MessageType, ServerInfo};

use crate::{capabilities::get_server_capabilities, server::{Client, LanguageServer, Result}};

pub struct Backend {

}

impl Backend {
    pub fn new() -> Self {
        Self {}
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
        client.log_message(MessageType::LOG, "Language server initialized.".to_string())
    }
}