use pxp_span::Span;

use crate::{FileId, HasFileId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    file: FileId,
    span: Span,
}

impl Location {
    pub fn new(file: FileId, span: Span) -> Self {
        Self { file, span }
    }
}

impl HasFileId for Location {
    fn file_id(&self) -> FileId {
        self.file
    }
}
