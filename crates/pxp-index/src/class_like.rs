use pxp_ast::{ClassModifierGroup, ConstantModifierGroup, MethodModifierGroup, Name, PropertyModifierGroup};
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
    pub traits: Vec<Symbol>,
    pub properties: Vec<Property>,
    pub methods: Vec<Method>,
    pub cases: Vec<Symbol>,
    pub constants: Vec<ClassConstant>,
    pub modifiers: ClassModifierGroup,
    pub kind: ClassKind,
}

#[derive(Debug, Clone)]
pub struct ClassConstant {
    pub name: Symbol,
    pub r#type: Type<Name>,
    pub modifiers: ConstantModifierGroup,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub(crate) enum ClassKind {
    #[default]
    Class,
    Interface,
    Trait,
    Enum,
}

impl ClassLike {
    pub fn new(name: Symbol, short: Symbol, namespace: Option<Symbol>, kind: ClassKind) -> Self {
        ClassLike {
            name,
            short,
            namespace,
            kind,
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