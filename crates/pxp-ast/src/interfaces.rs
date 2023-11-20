use crate::attributes::AttributeGroup;
use crate::classes::ClassishMember;
use crate::identifiers::SimpleIdentifier;

use crate::utils::CommaSeparated;
use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct InterfaceExtends {
    pub extends: Span,                             // `extends`
    pub parents: CommaSeparated<SimpleIdentifier>, // `Foo`, `Bar`
}
#[derive(Debug, PartialEq, Eq, Clone)]

pub struct InterfaceBody {
    pub left_brace: Span,              // `{`
    pub members: Vec<ClassishMember>, // `public const FOO = 123;`, `public function foo(): void;`
    pub right_brace: Span,             // `}`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct InterfaceStatement {
    pub attributes: Vec<AttributeGroup>,   // `#[Foo]`
    pub interface: Span,                   // `interface`
    pub name: SimpleIdentifier,            // `Foo`
    pub extends: Option<InterfaceExtends>, // `extends Bar`
    pub body: InterfaceBody,               // `{ ... }`
}
