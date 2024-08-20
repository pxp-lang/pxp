use lsp_types::{Hover, Position, Uri};

use crate::backend::Backend;

impl Backend {
    pub fn generate_hover(&self, uri: &Uri, position: &Position) -> Option<Hover> {
        None
    }
}