mod lsp;
mod capabilities;
mod state;
mod commands {
    pub mod hover;
}

use tower_lsp::{LspService, Server};
use lsp::Backend;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::build(Backend::new).finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
