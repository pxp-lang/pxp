use pxp_bytestring::ByteStr;

use crate::entities::{ClassEntity, ClassEntityKind};

use super::ReflectionMethod;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReflectionClass<'a> {
    entity: &'a ClassEntity,
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
}
