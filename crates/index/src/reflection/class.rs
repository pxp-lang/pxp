use pxp_bytestring::ByteStr;

use crate::{
    entities::{ClassEntity, ClassEntityKind},
    location::{HasLocation, Location},
};

use super::ReflectionMethod;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReflectionClass<'a> {
    entity: &'a ClassEntity,
}

impl<'a> HasLocation for ReflectionClass<'a> {
    fn location(&self) -> Location {
        self.entity.location
    }
}

impl<'a> ReflectionClass<'a> {
    pub fn new(entity: &'a ClassEntity) -> Self {
        Self { entity }
    }

    pub fn name(&self) -> &ByteStr {
        self.entity.name.resolved.as_ref()
    }

    pub fn short_name(&self) -> &ByteStr {
        self.entity.name.original.as_ref()
    }

    pub fn is_class(&self) -> bool {
        self.entity.kind == ClassEntityKind::Class
    }

    pub fn is_interface(&self) -> bool {
        self.entity.kind == ClassEntityKind::Interface
    }

    pub fn is_enum(&self) -> bool {
        self.entity.kind == ClassEntityKind::Enum
    }

    pub fn is_trait(&self) -> bool {
        self.entity.kind == ClassEntityKind::Trait
    }

    pub fn get_methods(&self) -> Vec<ReflectionMethod> {
        self.entity
            .methods
            .iter()
            .map(|m| ReflectionMethod::new(m, self))
            .collect()
    }

    pub fn get_method(&self, name: &ByteStr) -> Option<ReflectionMethod> {
        self.get_methods()
            .into_iter()
            .find(|method| method.get_name() == name)
    }

    pub fn get_static_methods(&self) -> Vec<ReflectionMethod> {
        self.get_methods()
            .into_iter()
            .filter(|method| method.is_static())
            .collect()
    }

    pub fn get_static_method(&self, name: &ByteStr) -> Option<ReflectionMethod> {
        self.get_static_methods()
            .into_iter()
            .find(|method| method.get_name() == name)
    }
}
