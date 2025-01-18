use pxp_ast::{Name, ResolvedName};
use pxp_bytestring::ByteStr;
use pxp_type::Type;

use crate::{
    location::{HasLocation, Location},
    FunctionEntity,
};

use super::{parameters::{CanReflectParameters, ReflectionParameter, ReflectsParameters}, ReflectionType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReflectionFunction<'a> {
    pub(crate) entity: &'a FunctionEntity,
}

impl<'a> ReflectionFunction<'a> {
    pub fn new(entity: &'a FunctionEntity) -> Self {
        Self { entity }
    }

    pub fn get_name(&self) -> &ByteStr {
        self.entity.name.resolved.as_ref()
    }

    pub fn get_short_name(&self) -> &ByteStr {
        self.entity.name.original.as_ref()
    }

    pub fn in_namespace(&self) -> bool {
        self.entity.name.resolved != self.entity.name.original
    }
}

impl<'a> HasLocation for ReflectionFunction<'a> {
    fn location(&self) -> Location {
        self.entity.location
    }
}

impl CanReflectParameters for ReflectionFunction<'_> {}

impl<'a> ReflectsParameters<'a, ReflectionFunction<'a>> for ReflectionFunction<'a> {
    fn get_parameters(&self) -> Vec<ReflectionParameter<'a, ReflectionFunction<'a>>> {
        self.entity
            .parameters
            .iter()
            .map(|p| ReflectionParameter::new(p, *self))
            .collect()
    }
}

impl IsFunctionLike for ReflectionFunction<'_> {}

impl<'a> ReflectionFunctionLike<'a> for ReflectionFunction<'a> {
    fn get_return_type(&self) -> Option<ReflectionType<'a>> {
        self.entity.return_type.as_ref().map(|t| ReflectionType::new(t))
    }

    fn returns_reference(&self) -> bool {
        self.entity.returns_reference
    }
}

pub trait IsFunctionLike {}

pub trait ReflectionFunctionLike<'a>: IsFunctionLike {
    fn get_return_type(&self) -> Option<ReflectionType<'a>>;

    fn has_return_type(&self) -> bool {
        self.get_return_type().is_some()
    }

    fn returns_reference(&self) -> bool;
}
