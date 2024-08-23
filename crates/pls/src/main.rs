use std::error::Error;

use backend::Backend;
use server::ServerManager;

mod capabilities;
mod server;
mod backend;
mod commands {
    pub mod document_symbol;
    pub mod hover;
    pub mod completion;
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ServerManager::new(|| Backend::new()).run()
}
