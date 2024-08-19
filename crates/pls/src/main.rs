use std::error::Error;

use backend::Backend;
use server::ServerManager;

mod capabilities;
mod server;
mod backend;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ServerManager::new(|| Backend::new()).run()
}
