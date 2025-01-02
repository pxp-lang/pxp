use pxp_ast::Name;
use pxp_bytestring::ByteStr;
use pxp_type::Type;

use crate::FunctionEntity;

use super::parameters::{CanReflectParameters, ReflectionParameter, ReflectsParameters};

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
    fn get_return_type(&self) -> Option<Type<Name>> {
        self.entity.return_type.clone()
    }

    fn returns_reference(&self) -> bool {
        self.entity.returns_reference
    }
}

pub(crate) trait IsFunctionLike {}

pub trait ReflectionFunctionLike<'a>: IsFunctionLike {
    fn get_return_type(&self) -> Option<Type<Name>>;

    fn has_return_type(&self) -> bool {
        self.get_return_type().is_some()
    }

    fn returns_reference(&self) -> bool;
}
