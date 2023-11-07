use pxp_span::Span;
use crate::node::Node;
use crate::attributes::AttributeGroup;
use crate::data_type::Type;
use crate::modifiers::PropertyModifierGroup;
use crate::variables::SimpleVariable;
use crate::Expression;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Property {
    pub attributes: Vec<AttributeGroup>,

    pub modifiers: PropertyModifierGroup,
    pub r#type: Option<Type>,
    pub entries: Vec<PropertyEntry>,
    pub end: Span,
}

impl Node for Property {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![];
        if let Some(r#type) = &mut self.r#type {
            children.push(r#type);
        }
        children.extend(
            self.entries
                .iter_mut()
                .map(|e| e as &mut dyn Node)
                .collect::<Vec<&mut dyn Node>>(),
        );
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct VariableProperty {
    pub attributes: Vec<AttributeGroup>,
    pub r#type: Option<Type>,
    pub entries: Vec<PropertyEntry>,
    pub end: Span,
}

impl Node for VariableProperty {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![];
        if let Some(r#type) = &mut self.r#type {
            children.push(r#type);
        }
        children.extend(
            self.entries
                .iter_mut()
                .map(|e| e as &mut dyn Node)
                .collect::<Vec<&mut dyn Node>>(),
        );
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum PropertyEntry {
    Uninitialized {
        variable: SimpleVariable,
    },
    Initialized {
        variable: SimpleVariable,
        equals: Span,
        value: Expression,
    },
}

impl Node for PropertyEntry {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            PropertyEntry::Uninitialized { variable } => vec![variable],
            PropertyEntry::Initialized {
                variable, value, ..
            } => vec![variable, value],
        }
    }
}

impl PropertyEntry {
    pub fn variable(&self) -> &SimpleVariable {
        match self {
            PropertyEntry::Uninitialized { variable } => variable,
            PropertyEntry::Initialized { variable, .. } => variable,
        }
    }
}
