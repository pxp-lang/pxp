use pxp_span::Span;
use crate::node::Node;
use crate::attributes::AttributeGroup;
use crate::constant::ClassishConstant;
use crate::functions::AbstractConstructor;
use crate::functions::AbstractMethod;
use crate::functions::ConcreteConstructor;
use crate::functions::ConcreteMethod;
use crate::identifiers::SimpleIdentifier;
use crate::modifiers::VisibilityModifier;
use crate::properties::Property;
use crate::properties::VariableProperty;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum TraitMember {
    Constant(ClassishConstant),
    TraitUsage(TraitUsage),
    Property(Property),
    VariableProperty(VariableProperty),
    AbstractMethod(AbstractMethod),
    AbstractConstructor(AbstractConstructor),
    ConcreteMethod(ConcreteMethod),
    ConcreteConstructor(ConcreteConstructor),
}

impl Node for TraitMember {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            TraitMember::Constant(constant) => vec![constant],
            TraitMember::TraitUsage(usage) => vec![usage],
            TraitMember::Property(property) => vec![property],
            TraitMember::VariableProperty(property) => vec![property],
            TraitMember::AbstractMethod(method) => vec![method],
            TraitMember::AbstractConstructor(constructor) => vec![constructor],
            TraitMember::ConcreteMethod(method) => vec![method],
            TraitMember::ConcreteConstructor(constructor) => vec![constructor],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct TraitBody {
    pub left_brace: Span,
    pub members: Vec<TraitMember>,
    pub right_brace: Span,
}

impl Node for TraitBody {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.members
            .iter_mut()
            .map(|member| member as &mut dyn Node)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct TraitStatement {
    pub r#trait: Span,
    pub name: SimpleIdentifier,
    pub attributes: Vec<AttributeGroup>,
    pub body: TraitBody,
}

impl Node for TraitStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.name, &mut self.body]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct TraitUsage {
    pub r#use: Span,
    pub traits: Vec<SimpleIdentifier>,
    pub adaptations: Vec<TraitUsageAdaptation>,
}

impl Node for TraitUsage {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.traits.iter_mut().map(|t| t as &mut dyn Node).collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum TraitUsageAdaptation {
    Alias {
        r#trait: Option<SimpleIdentifier>,
        method: SimpleIdentifier,
        alias: SimpleIdentifier,
        visibility: Option<VisibilityModifier>,
    },
    Visibility {
        r#trait: Option<SimpleIdentifier>,
        method: SimpleIdentifier,
        visibility: VisibilityModifier,
    },
    Precedence {
        r#trait: Option<SimpleIdentifier>,
        method: SimpleIdentifier,
        insteadof: Vec<SimpleIdentifier>,
    },
}
