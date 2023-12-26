use std::fmt::Display;

use crate::attributes::AttributeGroup;
use crate::classes::ClassishMember;

use crate::identifiers::SimpleIdentifier;

use crate::Expression;
use pxp_span::Span;
use pxp_syntax::backed_enum_type::BackedEnumType;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct UnitEnumCase {
    pub attributes: Vec<AttributeGroup>, // `#[Foo]`
    pub start: Span,                     // `case`
    pub name: SimpleIdentifier,          // `Bar`
    pub end: Span,                       // `;`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum UnitEnumMember {
    Case(UnitEnumCase),
    Classish(ClassishMember),
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct UnitEnumBody {
    pub left_brace: Span,             // `{`
    pub members: Vec<UnitEnumMember>, // `...`
    pub right_brace: Span,            // `}`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct UnitEnumStatement {
    pub attributes: Vec<AttributeGroup>,   // `#[Foo]`
    pub r#enum: Span,                      // `enum`
    pub name: SimpleIdentifier,            // `Foo`
    pub implements: Vec<SimpleIdentifier>, // `implements Bar`
    pub body: UnitEnumBody,                // `{ ... }`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct BackedEnumCase {
    pub attributes: Vec<AttributeGroup>, // `#[Foo]`
    pub case: Span,                      // `case`
    pub name: SimpleIdentifier,          // `Bar`
    pub equals: Span,                    // `=`
    pub value: Expression,               // `123`
    pub semicolon: Span,                 // `;`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum BackedEnumMember {
    Case(BackedEnumCase),
    Classish(ClassishMember),
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct BackedEnumBody {
    pub left_brace: Span,               // `{`
    pub members: Vec<BackedEnumMember>, // `...`
    pub right_brace: Span,              // `}`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct BackedEnumStatement {
    pub attributes: Vec<AttributeGroup>,   // `#[Foo]`
    pub r#enum: Span,                      // `enum`
    pub name: SimpleIdentifier,            // `Foo`
    pub backed_type: BackedEnumType,       // `: string`
    pub implements: Vec<SimpleIdentifier>, // `implements Bar`
    pub body: BackedEnumBody,              // `{ ... }`
}
