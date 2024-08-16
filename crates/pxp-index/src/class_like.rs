use pxp_ast::{
    ClassModifierGroup, ConstantModifierGroup, MethodModifierGroup, Name, PropertyModifierGroup,
};
use pxp_bytestring::ByteString;

use pxp_type::Type;

use crate::parameter::Parameter;

#[derive(Debug, Clone, Default)]
pub struct ClassLike {
    pub name: ByteString,
    pub short: ByteString,
    pub namespace: Option<ByteString>,
    pub parent: Option<ByteString>,
    pub interfaces: Vec<ByteString>,
    pub traits: Vec<ByteString>,
    pub properties: Vec<Property>,
    pub methods: Vec<Method>,
    pub cases: Vec<ByteString>,
    pub constants: Vec<ClassConstant>,
    pub modifiers: ClassModifierGroup,
    pub kind: ClassKind,
}

#[derive(Debug, Clone)]
pub struct ClassConstant {
    pub name: ByteString,
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
    pub fn new(name: ByteString, short: ByteString, namespace: Option<ByteString>, kind: ClassKind) -> Self {
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
    pub name: ByteString,
    pub r#type: Type<Name>,
    pub default: bool,
    pub modifiers: PropertyModifierGroup,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub name: ByteString,
    pub return_type: Type<Name>,
    pub modifiers: MethodModifierGroup,
    pub parameters: Vec<Parameter>,
    pub r#abstract: bool,
}
