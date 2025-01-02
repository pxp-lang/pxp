use pxp_ast::Name;
use pxp_type::Type;

use crate::{location::Location, FileId, HasFileId};

use super::parameters::Parameters;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionEntity {
    pub(crate) name: Name,
    pub(crate) parameters: Parameters,
    pub(crate) return_type: Option<Type<Name>>,
    pub(crate) location: Location,
}

impl FunctionEntity {

}

impl HasFileId for FunctionEntity {
    fn file_id(&self) -> FileId {
        self.location.file_id()
    }
}
