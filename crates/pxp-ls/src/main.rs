mod lsp;
mod capabilities;

use tower_lsp::{LspService, Server};
use lsp::Backend;

#[tokio::main]
async fn main() {

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::build(|client| Backend::new(client))
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}