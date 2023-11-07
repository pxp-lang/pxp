use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::node::Node;
use crate::parser::ast::attributes::AttributeGroup;
use crate::parser::ast::constant::ClassishConstant;
use crate::parser::ast::functions::ConcreteMethod;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::ast::Expression;

use super::traits::TraitUsage;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum UnitEnumMember {
    Case(UnitEnumCase),         // `case Bar;`
    Method(ConcreteMethod),     // `public function foo(): void { ... }`
    Constant(ClassishConstant), // `public const FOO = 123;`
    TraitUsage(TraitUsage),
}

impl Node for UnitEnumMember {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            UnitEnumMember::Case(case) => vec![case],
            UnitEnumMember::Method(method) => vec![method],
            UnitEnumMember::Constant(constant) => vec![constant],
            UnitEnumMember::TraitUsage(trait_usage) => vec![trait_usage],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum BackedEnumType {
    String(Span, Span), // `:` + `string`
    Int(Span, Span),    // `:` + `int`
}

impl Node for BackedEnumType {
    //
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum BackedEnumMember {
    Case(BackedEnumCase),
    Method(ConcreteMethod),
    Constant(ClassishConstant),
    TraitUsage(TraitUsage),
}

impl Node for BackedEnumMember {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            BackedEnumMember::Case(case) => vec![case],
            BackedEnumMember::Method(method) => vec![method],
            BackedEnumMember::Constant(constant) => vec![constant],
            BackedEnumMember::TraitUsage(trait_usage) => vec![trait_usage],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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
