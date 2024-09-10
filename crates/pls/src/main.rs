use std::error::Error;

use backend::Backend;
use pxp_lsp::ServerManager;

mod backend;
mod capabilities;
mod commands {
    pub mod completion;
    pub mod document_symbol;
    pub mod hover;
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ServerManager::new(Backend::new).run()
}
