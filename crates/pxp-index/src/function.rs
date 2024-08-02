use pxp_ast::Name;
use pxp_symbol::Symbol;
use pxp_type::Type;

use crate::parameter::Parameter;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Symbol,
    pub short: Symbol,
    pub namespace: Option<Symbol>,
    pub parameters: Vec<Parameter>,
    pub return_type: Type<Name>,
    pub returns_by_reference: bool,
}
