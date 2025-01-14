use pxp_ast::{Name, ResolvedName};
use pxp_bytestring::ByteStr;
use pxp_type::Type;

use crate::{entities::MethodEntity, location::{HasLocation, Location}};

use super::{
    function::{IsFunctionLike, ReflectionFunctionLike},
    parameters::{CanReflectParameters, ReflectsParameters},
    ReflectionClass, ReflectionParameter,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReflectionMethod<'a> {
    pub(crate) entity: &'a MethodEntity,
    pub(crate) owner: &'a ReflectionClass<'a>,
}

impl<'a> HasLocation for ReflectionMethod<'a> {
    fn location(&self) -> Location {
        self.entity.location
    }
}

impl<'a> ReflectionMethod<'a> {
    pub fn new(entity: &'a MethodEntity, owner: &'a ReflectionClass<'a>) -> Self {
        Self { entity, owner }
    }

    pub fn get_name(&self) -> &ByteStr {
        self.entity.name.symbol.as_ref()
    }

    pub fn get_class(&self) -> &ReflectionClass<'a> {
        self.owner
    }

    pub fn is_public(&self) -> bool {
        self.entity.modifiers.is_public()
    }

    pub fn is_protected(&self) -> bool {
        self.entity.modifiers.is_protected()
    }

    pub fn is_private(&self) -> bool {
        self.entity.modifiers.is_private()
    }

    pub fn is_static(&self) -> bool {
        self.entity.modifiers.has_static()
    }

    pub fn is_final(&self) -> bool {
        self.entity.modifiers.has_final()
    }

    pub fn is_abstract(&self) -> bool {
        self.entity.modifiers.has_abstract()
    }
}

impl CanReflectParameters for ReflectionMethod<'_> {}

impl<'a> ReflectsParameters<'a, ReflectionMethod<'a>> for ReflectionMethod<'a> {
    fn get_parameters(&self) -> Vec<super::ReflectionParameter<'a, ReflectionMethod<'a>>> {
        self.entity
            .parameters
            .iter()
            .map(|p| ReflectionParameter::new(p, *self))
            .collect()
    }
}

impl IsFunctionLike for ReflectionMethod<'_> {}

impl<'a> ReflectionFunctionLike<'a> for ReflectionMethod<'a> {
    fn get_return_type(&self) -> Option<&Type<ResolvedName>> {
        self.entity.return_type.as_ref()
    }

    fn returns_reference(&self) -> bool {
        self.entity.returns_reference
    }
}
