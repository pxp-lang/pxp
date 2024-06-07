use pxp_ast::{ClassModifierGroup, MethodModifierGroup, Name, PropertyModifierGroup};
use pxp_symbol::Symbol;
use pxp_type::Type;

use crate::parameter::Parameter;

#[derive(Debug, Clone)]
pub struct ClassLike {
    pub name: Symbol,
    pub short: Symbol,
    pub namespace: Option<Symbol>,
    pub properties: Vec<Property>,
    pub methods: Vec<Method>,
    pub modifiers: ClassModifierGroup,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: Symbol,
    pub r#type: Type<Name>,
    pub default: bool,
    pub modifiers: PropertyModifierGroup,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub name: Symbol,
    pub return_type: Type<Name>,
    pub modifiers: MethodModifierGroup,
    pub parameters: Vec<Parameter>,
}