use pxp_bytestring::ByteStr;

use crate::{
    location::{HasLocation, Location},
    Parameter,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReflectionParameter<'a, O: CanReflectParameters> {
    entity: &'a Parameter,
    owner: O,
}

impl<'a, O: CanReflectParameters> ReflectionParameter<'a, O> {
    pub fn new(entity: &'a Parameter, owner: O) -> Self {
        Self { entity, owner }
    }

    pub fn get_name(&self) -> &ByteStr {
        self.entity.name.stripped.as_ref()
    }

    pub fn has_type(&self) -> bool {
        self.entity.r#type.is_some()
    }

    pub fn is_optional(&self) -> bool {
        self.entity.optional
    }

    pub fn is_variadic(&self) -> bool {
        todo!()
    }
}

impl<'a, O: CanReflectParameters> HasLocation for ReflectionParameter<'a, O> {
    fn location(&self) -> Location {
        self.entity.location
    }
}

pub trait CanReflectParameters {}

pub trait ReflectsParameters<'a, O: CanReflectParameters>: CanReflectParameters {
    fn get_parameters(&self) -> Vec<ReflectionParameter<'a, O>>;

    fn get_number_of_parameters(&self) -> usize {
        self.get_parameters().len()
    }

    fn get_number_of_required_parameters(&self) -> usize {
        self.get_parameters()
            .iter()
            .filter(|p| !p.is_optional())
            .count()
    }

    fn is_variadic(&self) -> bool {
        self.get_parameters().iter().any(|p| p.is_variadic())
    }
}
