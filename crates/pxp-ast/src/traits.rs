use crate::attributes::AttributeGroup;
use crate::classes::ClassishMember;
use crate::identifiers::SimpleIdentifier;
use crate::modifiers::VisibilityModifier;
use crate::node::Node;
use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct TraitBody {
    pub left_brace: Span,
    pub members: Vec<ClassishMember>,
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
