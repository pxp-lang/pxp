use pxp_ast::Name;
use pxp_bytestring::ByteString;

use pxp_type::Type;

use crate::parameter::Parameter;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: ByteString,
    pub short: ByteString,
    pub namespace: Option<ByteString>,
    pub parameters: Vec<Parameter>,
    pub return_type: Type<Name>,
    pub returns_by_reference: bool,
}
