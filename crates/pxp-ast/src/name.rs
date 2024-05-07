use pxp_syntax::identifier::NameQualification;

use crate::identifiers::SimpleIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Name {
    pub identifier: SimpleIdentifier,
    pub qualification: NameQualification,
}