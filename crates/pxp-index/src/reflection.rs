#![allow(unused)]
use std::fmt::Debug;

use pxp_ast::Name;
use pxp_bytestring::ByteString;
use pxp_type::Type;

use crate::{
    class_like::{ClassConstant, ClassKind, ClassLike, Method, Property},
    constant::Constant,
    function::Function,
    parameter::Parameter,
    Index,
};

#[derive(Clone)]
pub struct ReflectionFunction {
    pub(crate) function: Function,
}

impl ReflectionFunction {
    pub fn get_name(&self) -> &ByteString {
        &self.function.name
    }

    pub fn get_short_name(&self) -> &ByteString {
        &self.function.short
    }

    pub fn get_namespace(&self) -> Option<&ByteString> {
        self.function.namespace.as_ref()
    }

    pub fn get_return_type(&self) -> &Type<Name> {
        &self.function.return_type
    }

    pub fn returns_by_reference(&self) -> bool {
        self.function.returns_by_reference
    }

    pub fn get_parameters(&self) -> Vec<ReflectionParameter> {
        self.function
            .parameters
            .iter()
            .map(|parameter| ReflectionParameter {
                parameter: parameter.clone(),
            })
            .collect()
    }

    pub fn get_parameter(&self, name: ByteString) -> Option<ReflectionParameter> {
        self.function
            .parameters
            .iter()
            .find(|parameter| parameter.name == name)
            .map(|parameter| ReflectionParameter {
                parameter: parameter.clone(),
            })
    }
}

#[derive(Clone)]
pub struct ReflectionParameter {
    pub(crate) parameter: Parameter,
}

impl ReflectionParameter {
    pub fn get_name(&self) -> &ByteString {
        &self.parameter.name
    }

    pub fn get_type(&self) -> &Type<Name> {
        &self.parameter.r#type
    }

    pub fn is_optional(&self) -> bool {
        self.parameter.default
    }

    pub fn is_passed_by_reference(&self) -> bool {
        self.parameter.reference
    }

    pub fn is_variadic(&self) -> bool {
        self.parameter.variadic
    }
}

#[derive(Clone)]
pub struct ReflectionClass {
    pub(crate) class: ClassLike,
}

impl ReflectionClass {
    pub fn is_final(&self) -> bool {
        self.class.modifiers.has_final()
    }

    pub fn is_abstract(&self) -> bool {
        self.class.modifiers.has_abstract()
    }

    pub fn is_readonly(&self) -> bool {
        self.class.modifiers.has_readonly()
    }

    pub fn is_enum(&self) -> bool {
        self.class.kind == ClassKind::Enum
    }

    pub fn is_class(&self) -> bool {
        self.class.kind == ClassKind::Class
    }

    pub fn is_interface(&self) -> bool {
        self.class.kind == ClassKind::Interface
    }

    pub fn is_trait(&self) -> bool {
        self.class.kind == ClassKind::Trait
    }

    pub fn get_properties(&self, index: &Index) -> Vec<ReflectionProperty> {
        let mut properties = self.get_own_properties();

        // If we have a parent class, we can access those public properties as well.
        if let Some(parent) = self.get_parent(index) {
            properties.extend(parent.get_properties(index));
        }

        // If we're using traits on this object, we also need to get the public properties from those.
        for r#trait in self.get_traits(index) {
            properties.extend(r#trait.get_properties(index));
        }

        properties
    }

    pub fn get_own_properties(&self) -> Vec<ReflectionProperty> {
        self.class
            .properties
            .iter()
            .map(|property| ReflectionProperty {
                class: self.clone(),
                property: property.clone(),
            })
            .collect()
    }

    pub fn get_property(&self, name: &ByteString) -> Option<ReflectionProperty> {
        self.class
            .properties
            .iter()
            .find(|property| &property.name == name)
            .map(|property| ReflectionProperty {
                class: self.clone(),
                property: property.clone(),
            })
    }

    /// Get properties that are accessible on this object from the given scope.
    /// 
    /// The `scope` is the class that we're currently inside of, based on AST location.
    pub fn get_accessible_properties(&self, scope: Option<&ReflectionClass>, index: &Index) -> Vec<ReflectionProperty> {
        // If we're not inside of an object, we can only access public properties.
        // FIXME: This isn't entirely true – we could be inside of a bound closure,
        //        but we don't have that information or intelligence just yet.
        let Some(scope) = scope else {
            return self.get_public_properties(index);
        };

        // If we reach this point, then we need to figure out what properties are accessible.
        // 1. If we're inside of the class we're trying to access, we can access all properties, e.g. $this.
        if self.get_name() == scope.get_name() {
            let mut properties = self.get_own_properties();

            if let Some(parent) = self.get_parent(index) {
                properties.extend(parent.get_public_properties(index));
                properties.extend(parent.get_protected_properties(index));
            }

            // We can also access all properties from traits.
            // This includes private properties since traits are essentially
            // copy-pasta'd into the class.
            for r#trait in self.get_traits(index) {
                properties.extend(r#trait.get_properties(index));
            }

            return properties;
        }

        // If we're not inside of the class we're trying to access, we can start building a list.
        // In all cases, we should be able to access the public properties of the class we're trying to access.
        let mut properties = self.get_public_properties(index);

        // 2. If the class we're trying to access is a parent class of the current scope, we can
        //    also access protected properties from the parent.
        if scope.has_parent(index, self) {
            properties.extend(self.get_protected_properties(index));
        }

        properties
    }

    pub fn get_public_properties(&self, index: &Index) -> Vec<ReflectionProperty> {
        self.get_properties(index)
            .into_iter()
            .filter(|property| property.is_public())
            .collect()
    }

    pub fn get_protected_properties(&self, index: &Index) -> Vec<ReflectionProperty> {
        self.get_properties(index)
            .into_iter()
            .filter(|property| property.is_protected())
            .collect()
    }

    pub fn get_private_properties(&self, index: &Index) -> Vec<ReflectionProperty> {
        self.get_properties(index)
            .into_iter()
            .filter(|property| property.is_private())
            .collect()
    }

    pub fn get_own_public_properties(&self) -> Vec<ReflectionProperty> {
        self.get_own_properties()
            .into_iter()
            .filter(|property| property.is_public())
            .collect()
    }

    pub fn get_own_protected_properties(&self) -> Vec<ReflectionProperty> {
        self.get_own_properties()
            .into_iter()
            .filter(|property| property.is_protected())
            .collect()
    }

    pub fn get_own_private_properties(&self) -> Vec<ReflectionProperty> {
        self.get_own_properties()
            .into_iter()
            .filter(|property| property.is_private())
            .collect()
    }

    pub fn get_name(&self) -> &ByteString {
        &self.class.name
    }

    pub fn get_short_name(&self) -> &ByteString {
        &self.class.short
    }

    pub fn get_namespace(&self) -> Option<&ByteString> {
        self.class.namespace.as_ref()
    }

    pub fn get_parent(&self, index: &Index) -> Option<ReflectionClass> {
        self.class
            .parent
            .as_ref()
            .and_then(|parent| index.get_class(parent))
    }

    pub fn has_parent(&self, index: &Index, other: &ReflectionClass) -> bool {
        let mut parent = self.get_parent(index);

        while let Some(p) = parent {
            if p.get_name() == other.get_name() {
                return true;
            }

            parent = p.get_parent(index);
        }

        false
    }

    pub fn get_interfaces(&self, index: &Index) -> Vec<ReflectionClass> {
        self.class
            .interfaces
            .iter()
            .filter_map(move |interface| index.get_class(interface))
            .collect()
    }

    pub fn get_traits(&self, index: &Index) -> Vec<ReflectionClass> {
        self.class
            .traits
            .iter()
            .filter_map(move |r#trait| index.get_class(r#trait))
            .collect()
    }

    pub fn get_own_methods(&self) -> Vec<ReflectionMethod> {
        self.class
            .methods
            .iter()
            .map(|method| ReflectionMethod {
                class: self.clone(),
                method: method.clone(),
            })
            .collect()
    }

    pub fn get_accessible_methods(&self, scope: Option<&ReflectionClass>, index: &Index) -> Vec<ReflectionMethod> {
        // If we're not inside of an object, we can only access public properties.
        // FIXME: This isn't entirely true – we could be inside of a bound closure,
        //        but we don't have that information or intelligence just yet.
        let Some(scope) = scope else {
            return self.get_public_methods(index);
        };

        // If we reach this point, then we need to figure out what properties are accessible.
        // 1. If we're inside of the class we're trying to access, we can access all properties, e.g. $this.
        if self.get_name() == scope.get_name() {
            let mut methods = self.get_own_methods();

            if let Some(parent) = self.get_parent(index) {
                methods.extend(parent.get_public_methods(index));
                methods.extend(parent.get_protected_methods(index));
            }

            // We can also access all methods from traits.
            // This includes private methods since traits are essentially
            // copy-pasta'd into the class.
            for r#trait in self.get_traits(index) {
                methods.extend(r#trait.get_methods(index));
            }

            return methods;
        }

        // If we're not inside of the class we're trying to access, we can start building a list.
        // In all cases, we should be able to access the public methods of the class we're trying to access.
        let mut methods = self.get_public_methods(index);

        // 2. If the class we're trying to access is a parent class of the current scope, we can
        //    also access protected methods from the parent.
        if scope.has_parent(index, self) {
            methods.extend(self.get_protected_methods(index));
        }

        methods
    }

    pub fn get_methods(&self, index: &Index) -> Vec<ReflectionMethod> {
        let mut methods = self.get_own_methods();

        if let Some(parent) = self.get_parent(index) {
            methods.extend(parent.get_methods(index));
        }

        for r#trait in self.get_traits(index) {
            methods.extend(r#trait.get_methods(index));
        }

        // Interfaces also provide methods, so we need to include thos as well,
        // even if those methods are abstract in nature.
        for r#interface in self.get_interfaces(index) {
            methods.extend(r#interface.get_methods(index));
        }

        methods
    }

    pub fn get_public_methods(&self, index: &Index) -> Vec<ReflectionMethod> {
        self.get_methods(index)
            .into_iter()
            .filter(|method| method.is_public())
            .collect()
    }

    pub fn get_protected_methods(&self, index: &Index) -> Vec<ReflectionMethod> {
        self.get_methods(index)
            .into_iter()
            .filter(|method| method.is_protected())
            .collect()
    }

    pub fn get_private_methods(&self, index: &Index) -> Vec<ReflectionMethod> {
        self.get_methods(index)
            .into_iter()
            .filter(|method| method.is_private())
            .collect()
    }

    pub fn get_method(&self, name: &ByteString) -> Option<ReflectionMethod> {
        self.class
            .methods
            .iter()
            .find(|method| &method.name == name)
            .map(|method| ReflectionMethod {
                class: self.clone(),
                method: method.clone(),
            })
    }

    pub fn get_static_method(&self, name: &ByteString) -> Option<ReflectionMethod> {
        self.class
            .methods
            .iter()
            .find(|method| &method.name == name && method.modifiers.has_static())
            .map(|method| ReflectionMethod {
                class: self.clone(),
                method: method.clone(),
            })
    }

    pub fn get_own_public_methods(&self) -> Vec<ReflectionMethod> {
        self.get_own_methods()
            .into_iter()
            .filter(|method| method.is_public())
            .collect()
    }

    pub fn get_own_protected_methods(&self) -> Vec<ReflectionMethod> {
        self.get_own_methods()
            .into_iter()
            .filter(|method| method.is_protected())
            .collect()
    }

    pub fn get_own_private_methods(&self) -> Vec<ReflectionMethod> {
        self.get_own_methods()
            .into_iter()
            .filter(|method| method.is_private())
            .collect()
    }

    pub fn get_cases(&self) -> Vec<ReflectionCase> {
        self.class
            .cases
            .iter()
            .map(|case| ReflectionCase {
                r#enum: self.clone(),
                case: case.clone(),
            })
            .collect()
    }

    pub fn get_constants(&self) -> Vec<ReflectionClassConstant> {
        self.class
            .constants
            .iter()
            .map(|constant| ReflectionClassConstant {
                class: self.clone(),
                constant: constant.clone(),
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct ReflectionClassConstant {
    pub(crate) class: ReflectionClass,
    pub(crate) constant: ClassConstant,
}

impl ReflectionClassConstant {
    pub fn get_name(&self) -> &ByteString {
        &self.constant.name
    }

    pub fn get_type(&self) -> &Type<Name> {
        &self.constant.r#type
    }

    pub fn is_public(&self) -> bool {
        self.constant.modifiers.is_public()
    }

    pub fn is_protected(&self) -> bool {
        self.constant.modifiers.is_protected()
    }

    pub fn is_private(&self) -> bool {
        self.constant.modifiers.is_private()
    }

    pub fn is_final(&self) -> bool {
        self.constant.modifiers.has_final()
    }
}

#[derive(Clone)]
pub struct ReflectionCase {
    pub(crate) r#enum: ReflectionClass,
    pub(crate) case: ByteString,
}

impl ReflectionCase {
    pub fn get_name(&self) -> &ByteString {
        &self.case
    }
}

impl Debug for ReflectionClass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReflectionClass")
            .field("name", &self.get_name())
            .field("short", &self.get_short_name())
            .field("namespace", &self.get_namespace())
            .field("properties", &self.get_own_properties())
            .field("methods", &self.get_own_methods())
            .finish()
    }
}

#[derive(Clone)]
pub struct ReflectionProperty {
    pub(crate) class: ReflectionClass,
    pub(crate) property: Property,
}

impl ReflectionProperty {
    pub fn get_name(&self) -> &ByteString {
        &self.property.name
    }

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

    pub fn get_type(&self) -> &Type<Name> {
        &self.property.r#type
    }
}

impl Debug for ReflectionProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReflectionProperty")
            .field("name", &self.property.name)
            .field("type", &self.property.r#type)
            .field("default", &self.property.default)
            .field("modifiers", &self.property.modifiers)
            .finish()
    }
}

#[derive(Clone)]
pub struct ReflectionMethod {
    pub(crate) class: ReflectionClass,
    pub(crate) method: Method,
}

impl ReflectionMethod {
    pub fn is_static(&self) -> bool {
        self.method.modifiers.has_static()
    }

    pub fn is_public(&self) -> bool {
        self.method.modifiers.is_public()
    }

    pub fn is_protected(&self) -> bool {
        self.method.modifiers.is_protected()
    }

    pub fn is_private(&self) -> bool {
        self.method.modifiers.is_private()
    }

    pub fn is_abstract(&self) -> bool {
        self.method.r#abstract || self.method.modifiers.has_abstract()
    }

    pub fn get_return_type(&self) -> &Type<Name> {
        &self.method.return_type
    }

    pub fn get_name(&self) -> &ByteString {
        &self.method.name
    }

    pub fn get_class(&self) -> &ReflectionClass {
        &self.class
    }
}

impl Debug for ReflectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReflectionMethod")
            .field("name", &self.method.name)
            .field("return_type", &self.method.return_type)
            .field("modifiers", &self.method.modifiers)
            .field("parameters", &self.method.parameters)
            .finish()
    }
}

#[derive(Clone)]
pub struct ReflectionConstant {
    pub(crate) constant: Constant,
}

impl ReflectionConstant {
    pub fn get_name(&self) -> &ByteString {
        &self.constant.name
    }

    pub fn get_short_name(&self) -> &ByteString {
        &self.constant.short
    }

    pub fn get_namespace(&self) -> Option<&ByteString> {
        self.constant.namespace.as_ref()
    }
}
