use pxp_ast::{ClassModifierGroup, MethodModifierGroup, Name, PropertyModifierGroup};
use pxp_symbol::Symbol;
use pxp_type::Type;

use crate::parameter::Parameter;

#[derive(Debug, Clone, Default)]
pub struct ClassLike {
    pub name: Symbol,
    pub short: Symbol,
    pub namespace: Option<Symbol>,
    pub parent: Option<Symbol>,
    pub interfaces: Vec<Symbol>,
    pub properties: Vec<Property>,
    pub methods: Vec<Method>,
    pub modifiers: ClassModifierGroup,
}

impl ClassLike {
    pub fn new(name: Symbol, short: Symbol, namespace: Option<Symbol>) -> Self {
        ClassLike {
            name,
            short,
            namespace,
            ..Default::default()
        }
    }
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
    pub r#abstract: bool,
}