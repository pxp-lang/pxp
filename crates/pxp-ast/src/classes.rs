use std::slice::Iter;

use crate::attributes::AttributeGroup;
use crate::constant::ClassishConstant;
use crate::functions::AbstractConstructor;
use crate::functions::AbstractMethod;
use crate::functions::ConcreteConstructor;
use crate::functions::ConcreteMethod;
use crate::identifiers::SimpleIdentifier;
use crate::modifiers::ClassModifierGroup;

use crate::name::Name;
use crate::properties::Property;
use crate::properties::VariableProperty;
use crate::traits::TraitUsage;
use crate::utils::CommaSeparated;
use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ClassBody {
    pub left_brace: Span, // `{`
    pub members: Vec<ClassishMember>,
    pub right_brace: Span, // `}`
}

impl ClassBody {
    pub fn iter(&self) -> Iter<'_, ClassishMember> {
        self.members.iter()
    }
}

impl IntoIterator for ClassBody {
    type Item = ClassishMember;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.members.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ClassStatement {
    pub attributes: Vec<AttributeGroup>,     // `#[Qux]`
    pub modifiers: ClassModifierGroup,       // `abstract`, `final`
    pub class: Span,                         // `class`
    pub name: Name,                          // `Foo`
    pub extends: Option<ClassExtends>,       // `extends Foo`
    pub implements: Option<ClassImplements>, // `implements Bar, Baz`
    pub body: ClassBody,                     // `{ ... }`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct AnonymousClassBody {
    pub left_brace: Span, // `{`
    pub members: Vec<ClassishMember>,
    pub right_brace: Span, // `}`
}

impl AnonymousClassBody {
    pub fn iter(&self) -> Iter<'_, ClassishMember> {
        self.members.iter()
    }
}

impl IntoIterator for AnonymousClassBody {
    type Item = ClassishMember;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.members.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct AnonymousClassExpression {
    pub attributes: Vec<AttributeGroup>,     // `#[Qux]`
    pub class: Span,                         // `class`
    pub extends: Option<ClassExtends>,       // `extends Foo`
    pub implements: Option<ClassImplements>, // `implements Baz, Baz`
    pub body: AnonymousClassBody,            // `{ ... }`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ClassExtends {
    pub extends: Span,            // `extends`
    pub parent: SimpleIdentifier, // `Foo`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ClassImplements {
    pub implements: Span,                             // `implements`
    pub interfaces: CommaSeparated<SimpleIdentifier>, // `Bar, Baz`
}

impl ClassImplements {
    pub fn iter(&self) -> Iter<'_, SimpleIdentifier> {
        self.interfaces.iter()
    }
}

impl IntoIterator for ClassImplements {
    type Item = SimpleIdentifier;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.interfaces.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum ClassishMember {
    Constant(ClassishConstant),
    TraitUsage(TraitUsage),
    Property(Property),
    VariableProperty(VariableProperty),
    AbstractMethod(AbstractMethod),
    AbstractConstructor(AbstractConstructor),
    ConcreteMethod(ConcreteMethod),
    ConcreteConstructor(ConcreteConstructor),
}
