use lsp_types::{InitializeParams, InitializeResult, ServerInfo};

use crate::{capabilities::get_server_capabilities, server::LanguageServer};

pub struct Backend {

}

impl Backend {
    pub fn new() -> Self {
        Self {}
    }
}

impl LanguageServer for Backend {
    fn initialize(&mut self, _: &InitializeParams) -> InitializeResult {
        InitializeResult {
            capabilities: get_server_capabilities(),
            server_info: Some(ServerInfo {
                name: "PLS (PHP Language Server)".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        }
    }
}