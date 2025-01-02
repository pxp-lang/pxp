use pxp_ast::ResolvedName;

use crate::{location::Location, HasFileId};

#[derive(Debug, Clone, PartialEq)]
pub struct ClassEntity {
    pub(crate) name: ResolvedName,
    pub(crate) location: Location,
}

impl HasFileId for ClassEntity {
    fn file_id(&self) -> crate::FileId {
        self.location.file_id()
    }
}
