use pxp_ast::ResolvedName;
use pxp_type::Type;

pub struct ReflectionType<'a> {
    entity: &'a Type<ResolvedName>,
}

impl<'a> ReflectionType<'a> {
    pub fn new(entity: &'a Type<ResolvedName>) -> Self {
        Self { entity }
    }

    pub fn allows_null(&self) -> bool {
        self.entity.allows_null()
    }

    pub fn is(&self, other: &Type<ResolvedName>) -> bool {
        self.entity == other
    }

    pub fn to_type(&self) -> &Type<ResolvedName> {
        self.entity
    }
}
