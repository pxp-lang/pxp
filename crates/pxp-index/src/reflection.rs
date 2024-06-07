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

    pub fn get_properties(&self) -> impl Iterator<Item = ReflectionParameter> + 'a {
        self.class.properties.iter().map(|parameter| ReflectionParameter { parameter, index: self.index })
    }

    pub fn get_property(&self, name: Symbol) -> Option<ReflectionParameter> {
        self.class.properties.iter().find(|parameter| parameter.name == name).map(|parameter| ReflectionParameter { parameter, index: self.index })
    }

    pub fn get_name(&self) -> Symbol {
        self.class.name
    }

    pub fn get_short_name(&self) -> Symbol {
        self.class.short
    }

    pub fn get_methods(&self) -> impl Iterator<Item = ReflectionMethod> + 'a {
        self.class.methods.iter().map(|method| ReflectionMethod { method, index: self.index })
    }

    pub fn get_method(&self, name: Symbol) -> Option<ReflectionMethod> {
        self.class.methods.iter().find(|method| method.name == name).map(|method| ReflectionMethod { method, index: self.index })
    }
}

#[derive(Debug, Clone)]
pub struct ReflectionParameter<'a> {
    pub(crate) parameter: &'a Property,
    pub(crate) index: &'a Index,
}

#[derive(Debug, Clone)]
pub struct ReflectionMethod<'a> {
    pub(crate) method: &'a Method,
    pub(crate) index: &'a Index,
}