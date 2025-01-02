use crate::entities::ClassEntity;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReflectionClass<'a> {
    entity: &'a ClassEntity,
}

impl<'a> ReflectionClass<'a> {
    pub fn new(entity: &'a ClassEntity) -> Self {
        Self { entity }
    }
}
