use crate::attributes::AttributeGroup;
use crate::classes::ClassishMember;


use crate::identifiers::SimpleIdentifier;
use crate::node::Node;
use crate::Expression;
use pxp_span::Span;



#[derive(Debug, PartialEq, Eq, Clone)]

pub struct UnitEnumCase {
    pub attributes: Vec<AttributeGroup>, // `#[Foo]`
    pub start: Span,                     // `case`
    pub name: SimpleIdentifier,          // `Bar`
    pub end: Span,                       // `;`
}

impl Node for UnitEnumCase {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.name]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum UnitEnumMember {
    Case(UnitEnumCase),
    Classish(ClassishMember),
}

impl Node for UnitEnumMember {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            UnitEnumMember::Case(case) => vec![case],
            UnitEnumMember::Classish(classish) => vec![classish],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct UnitEnumBody {
    pub left_brace: Span,             // `{`
    pub members: Vec<UnitEnumMember>, // `...`
    pub right_brace: Span,            // `}`
}

impl Node for UnitEnumBody {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.members
            .iter_mut()
            .map(|m| m as &mut dyn Node)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct UnitEnumStatement {
    pub attributes: Vec<AttributeGroup>,   // `#[Foo]`
    pub r#enum: Span,                      // `enum`
    pub name: SimpleIdentifier,            // `Foo`
    pub implements: Vec<SimpleIdentifier>, // `implements Bar`
    pub body: UnitEnumBody,                // `{ ... }`
}

impl Node for UnitEnumStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.name];
        for implement in &mut self.implements {
            children.push(implement);
        }
        children.push(&mut self.body);
        children
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]

pub enum BackedEnumType {
    String(Span, Span), // `:` + `string`
    Int(Span, Span),    // `:` + `int`
}

impl Node for BackedEnumType {
    //
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

impl Node for BackedEnumCase {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.name, &mut self.value]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum BackedEnumMember {
    Case(BackedEnumCase),
    Classish(ClassishMember),
}

impl Node for BackedEnumMember {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            BackedEnumMember::Case(case) => vec![case],
            BackedEnumMember::Classish(classish) => vec![classish],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct BackedEnumBody {
    pub left_brace: Span,               // `{`
    pub members: Vec<BackedEnumMember>, // `...`
    pub right_brace: Span,              // `}`
}

impl Node for BackedEnumBody {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.members
            .iter_mut()
            .map(|m| m as &mut dyn Node)
            .collect()
    }
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

impl Node for BackedEnumStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.name, &mut self.backed_type];
        for implement in &mut self.implements {
            children.push(implement);
        }
        children.push(&mut self.body);
        children
    }
}
