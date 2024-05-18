use crate::attributes::AttributeGroup;
use crate::classes::ClassishMember;
use crate::identifiers::SimpleIdentifier;
use crate::modifiers::VisibilityModifier;
use crate::name::Name;

use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct TraitBody {
    pub left_brace: Span,
    pub members: Vec<ClassishMember>,
    pub right_brace: Span,
}
#[derive(Debug, PartialEq, Eq, Clone)]

pub struct TraitStatement {
    pub r#trait: Span,
    pub name: Name,
    pub attributes: Vec<AttributeGroup>,
    pub body: TraitBody,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct TraitUsage {
    pub r#use: Span,
    pub traits: Vec<SimpleIdentifier>,
    pub adaptations: Vec<TraitUsageAdaptation>,
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
