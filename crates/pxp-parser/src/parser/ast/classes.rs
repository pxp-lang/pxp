use std::slice::Iter;

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::node::Node;
use crate::parser::ast::attributes::AttributeGroup;
use crate::parser::ast::constant::ClassishConstant;
use crate::parser::ast::functions::AbstractConstructor;
use crate::parser::ast::functions::AbstractMethod;
use crate::parser::ast::functions::ConcreteConstructor;
use crate::parser::ast::functions::ConcreteMethod;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::ast::modifiers::ClassModifierGroup;
use crate::parser::ast::properties::Property;
use crate::parser::ast::properties::VariableProperty;
use crate::parser::ast::traits::TraitUsage;
use crate::parser::ast::utils::CommaSeparated;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ClassBody {
    pub left_brace: Span, // `{`
    pub members: Vec<ClassMember>,
    pub right_brace: Span, // `}`
}

impl ClassBody {
    pub fn iter(&self) -> Iter<'_, ClassMember> {
        self.members.iter()
    }
}

impl IntoIterator for ClassBody {
    type Item = ClassMember;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.members.into_iter()
    }
}

impl Node for ClassBody {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.members
            .iter_mut()
            .map(|member| member as &mut dyn Node)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ClassStatement {
    pub attributes: Vec<AttributeGroup>, // `#[Qux]`
    #[serde(flatten)]
    pub modifiers: ClassModifierGroup, // `abstract`, `final`
    pub class: Span,                     // `class`
    pub name: SimpleIdentifier,          // `Foo`
    pub extends: Option<ClassExtends>,   // `extends Foo`
    pub implements: Option<ClassImplements>, // `implements Bar, Baz`
    pub body: ClassBody,                 // `{ ... }`
}

impl Node for ClassStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.name];
        if let Some(extends) = &mut self.extends {
            children.push(extends);
        }
        if let Some(implements) = &mut self.implements {
            children.push(implements);
        }
        children.push(&mut self.body);
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct AnonymousClassBody {
    pub left_brace: Span, // `{`
    pub members: Vec<AnonymousClassMember>,
    pub right_brace: Span, // `}`
}

impl AnonymousClassBody {
    pub fn iter(&self) -> Iter<'_, AnonymousClassMember> {
        self.members.iter()
    }
}

impl IntoIterator for AnonymousClassBody {
    type Item = AnonymousClassMember;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.members.into_iter()
    }
}

impl Node for AnonymousClassBody {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.members
            .iter_mut()
            .map(|member| member as &mut dyn Node)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct AnonymousClassExpression {
    pub attributes: Vec<AttributeGroup>,     // `#[Qux]`
    pub class: Span,                         // `class`
    pub extends: Option<ClassExtends>,       // `extends Foo`
    pub implements: Option<ClassImplements>, // `implements Baz, Baz`
    pub body: AnonymousClassBody,            // `{ ... }`
}

impl Node for AnonymousClassExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![];
        if let Some(extends) = &mut self.extends {
            children.push(extends);
        }
        if let Some(implements) = &mut self.implements {
            children.push(implements);
        }
        children.push(&mut self.body);
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ClassExtends {
    pub extends: Span,            // `extends`
    pub parent: SimpleIdentifier, // `Foo`
}

impl Node for ClassExtends {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.parent]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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

impl Node for ClassImplements {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.interfaces.children()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum ClassMember {
    Constant(ClassishConstant),
    TraitUsage(TraitUsage),
    Property(Property),
    VariableProperty(VariableProperty),
    AbstractMethod(AbstractMethod),
    AbstractConstructor(AbstractConstructor),
    ConcreteMethod(ConcreteMethod),
    ConcreteConstructor(ConcreteConstructor),
}

impl Node for ClassMember {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            ClassMember::Constant(constant) => vec![constant],
            ClassMember::TraitUsage(usage) => vec![usage],
            ClassMember::Property(property) => vec![property],
            ClassMember::VariableProperty(property) => vec![property],
            ClassMember::AbstractMethod(method) => vec![method],
            ClassMember::AbstractConstructor(method) => vec![method],
            ClassMember::ConcreteMethod(method) => vec![method],
            ClassMember::ConcreteConstructor(method) => vec![method],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum AnonymousClassMember {
    Constant(ClassishConstant),
    TraitUsage(TraitUsage),
    Property(Property),
    VariableProperty(VariableProperty),
    ConcreteMethod(ConcreteMethod),
    ConcreteConstructor(ConcreteConstructor),
}

impl Node for AnonymousClassMember {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            AnonymousClassMember::Constant(constant) => vec![constant],
            AnonymousClassMember::TraitUsage(usage) => vec![usage],
            AnonymousClassMember::Property(property) => vec![property],
            AnonymousClassMember::VariableProperty(property) => vec![property],
            AnonymousClassMember::ConcreteMethod(method) => vec![method],
            AnonymousClassMember::ConcreteConstructor(method) => vec![method],
        }
    }
}
