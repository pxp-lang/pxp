use pxp_ast::Name;
use pxp_bytestring::ByteString;
use pxp_type::Type;

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: ByteString,
    pub r#type: Type<Name>,
    pub default: bool,
    pub variadic: bool,
    pub reference: bool,
}
