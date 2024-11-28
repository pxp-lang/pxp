# LSP

This crate provides a more convenient API around the excellent [lsp-server](https://github.com/rust-lang/rust-analyzer/tree/master/lib/lsp-server) crate developed by the `rust-analyzer` team.

The original crate is quite rugged and barebones. It leaves us, the developers, to manually wire up requests, responses and notifications without a clearly defined API or standard.

To combat this problem, I decided to develop a wrapper around the crate that focuses on developer-experience and simplicity.

## Usage

```rust
use std::error::Error;
use pxp_lsp::{
    ServerManager, LanguageServer,
    types::{InitializeParams, InitializeResult, ServerCapabilities, ServerInfo}
};

struct Backend;

impl Backend {
    pub fn new() -> Self {
        Self
    }
}

impl LanguageServer for Backend {
    fn initialize(&mut self, _: &Client, _: &InitializeParams) -> InitializeResult {
        InitializeResult {
            capabilities: ServerCapabilities {
                // ...
            },
            server_info: Some(ServerInfo {
                name: String::from("My Language Server"),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            })
        }
    }

    // Implement your request and notification handlers here...
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ServerManager::new(Backend::new).run()
}
```