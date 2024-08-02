use pxp_ast::Name;
use pxp_symbol::Symbol;
use pxp_type::Type;

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: Symbol,
    pub r#type: Type<Name>,
    pub default: bool,
    pub variadic: bool,
    pub reference: bool,
}
