use tower_lsp::lsp_types::{Hover, Position, Url};

use crate::lsp::Backend;

impl Backend {
    pub async fn calculate_hovers(&self, uri: &Url, position: Position) -> Option<Hover> {
        todo!()
    }
}