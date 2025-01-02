use pxp_ast::ResolvedName;

use crate::{location::Location, HasFileId};

#[derive(Debug, Clone, PartialEq)]
pub struct ClassEntity {
    pub(crate) name: ResolvedName,
    pub(crate) kind: ClassEntityKind,
    pub(crate) location: Location,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClassEntityKind {
    Class,
    Interface,
    Enum,
    Trait,
}

impl HasFileId for ClassEntity {
    fn file_id(&self) -> crate::FileId {
        self.location.file_id()
    }
}
