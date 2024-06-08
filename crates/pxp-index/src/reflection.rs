use pxp_symbol::Symbol;

use crate::{class_like::{ClassLike, Method, Property}, Index};

#[derive(Debug, Clone)]
pub struct ReflectionClass<'a> {
    pub(crate) class: &'a ClassLike,
    pub(crate) index: &'a Index,
}

impl<'a> ReflectionClass<'a> {
    pub fn is_final(&self) -> bool {
        self.class.modifiers.has_final()
    }

    pub fn is_abstract(&self) -> bool {
        self.class.modifiers.has_abstract()
    }

    pub fn is_readonly(&self) -> bool {
        self.class.modifiers.has_readonly()
    }

    pub fn get_properties(&'a self) -> impl Iterator<Item = ReflectionProperty> + 'a {
        self.class.properties.iter().map(|property| ReflectionProperty { class: self, property, index: self.index })
    }

    pub fn get_property(&self, name: Symbol) -> Option<ReflectionProperty> {
        self.class.properties.iter().find(|property| property.name == name).map(|property| ReflectionProperty { class: self, property, index: self.index })
    }

    pub fn get_name(&self) -> Symbol {
        self.class.name
    }

    pub fn get_short_name(&self) -> Symbol {
        self.class.short
    }

    pub fn get_namespace(&self) -> Option<Symbol> {
        self.class.namespace
    }

    pub fn get_methods(&'a self) -> impl Iterator<Item = ReflectionMethod> + 'a {
        self.class.methods.iter().map(|method| ReflectionMethod { class: self, method, index: self.index })
    }

    pub fn get_method(&self, name: Symbol) -> Option<ReflectionMethod> {
        self.class.methods.iter().find(|method| method.name == name).map(|method| ReflectionMethod { class: self, method, index: self.index })
    }
}

#[derive(Debug, Clone)]
pub struct ReflectionProperty<'a> {
    pub(crate) class: &'a ReflectionClass<'a>,
    pub(crate) property: &'a Property,
    pub(crate) index: &'a Index,
}

impl<'a> ReflectionProperty<'a> {
    pub fn is_static(&self) -> bool {
        self.property.modifiers.has_static()
    }

    pub fn is_readonly(&self) -> bool {
        self.property.modifiers.has_readonly()
    }

    pub fn is_public(&self) -> bool {
        self.property.modifiers.is_public()
    }

    pub fn is_protected(&self) -> bool {
        self.property.modifiers.is_protected()
    }

    pub fn is_private(&self) -> bool {
        self.property.modifiers.is_private()
    }
}

#[derive(Debug, Clone)]
pub struct ReflectionMethod<'a> {
    pub(crate) class: &'a ReflectionClass<'a>,
    pub(crate) method: &'a Method,
    pub(crate) index: &'a Index,
}