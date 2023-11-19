use crate::attributes::AttributeGroup;
use crate::classes::ClassishMember;
use crate::identifiers::SimpleIdentifier;
use crate::node::Node;
use crate::utils::CommaSeparated;
use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct InterfaceExtends {
    pub extends: Span,                             // `extends`
    pub parents: CommaSeparated<SimpleIdentifier>, // `Foo`, `Bar`
}

impl Node for InterfaceExtends {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.parents.children()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct InterfaceBody {
    pub left_brace: Span,              // `{`
    pub members: Vec<ClassishMember>, // `public const FOO = 123;`, `public function foo(): void;`
    pub right_brace: Span,             // `}`
}

impl Node for InterfaceBody {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.members
            .iter_mut()
            .map(|member| member as &mut dyn Node)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct InterfaceStatement {
    pub attributes: Vec<AttributeGroup>,   // `#[Foo]`
    pub interface: Span,                   // `interface`
    pub name: SimpleIdentifier,            // `Foo`
    pub extends: Option<InterfaceExtends>, // `extends Bar`
    pub body: InterfaceBody,               // `{ ... }`
}

impl Node for InterfaceStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.name];
        if let Some(extends) = &mut self.extends {
            children.push(extends);
        }
        children
    }
}
