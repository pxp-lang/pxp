use std::slice::Iter;

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::node::Node;
use crate::parser::ast::attributes::AttributeGroup;
use crate::parser::ast::comments::CommentGroup;
use crate::parser::ast::data_type::Type;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::ast::modifiers::MethodModifierGroup;
use crate::parser::ast::modifiers::PromotedPropertyModifierGroup;
use crate::parser::ast::utils::CommaSeparated;
use crate::parser::ast::variables::SimpleVariable;
use crate::parser::ast::Expression;
use crate::parser::ast::Statement;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ReturnType {
    pub colon: Span,
    pub data_type: Type,
}

impl Node for ReturnType {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.data_type]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct FunctionParameter {
    pub comments: CommentGroup,
    pub name: SimpleVariable,
    pub attributes: Vec<AttributeGroup>,
    pub data_type: Option<Type>,
    pub ellipsis: Option<Span>,
    pub default: Option<Expression>,
    pub ampersand: Option<Span>,
}

impl Node for FunctionParameter {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.name];
        if let Some(data_type) = &mut self.data_type {
            children.push(data_type);
        }
        if let Some(default) = &mut self.default {
            children.push(default);
        }
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct FunctionParameterList {
    pub comments: CommentGroup,
    pub left_parenthesis: Span,
    pub parameters: CommaSeparated<FunctionParameter>,
    pub right_parenthesis: Span,
}

impl FunctionParameterList {
    pub fn iter(&self) -> Iter<'_, FunctionParameter> {
        self.parameters.iter()
    }
}

impl IntoIterator for FunctionParameterList {
    type Item = FunctionParameter;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.parameters.into_iter()
    }
}

impl Node for FunctionParameterList {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.parameters.children()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct FunctionBody {
    pub comments: CommentGroup,
    pub left_brace: Span,
    pub statements: Vec<Statement>,
    pub right_brace: Span,
}

impl Node for FunctionBody {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.statements
            .iter_mut()
            .map(|x| x as &mut dyn Node)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct FunctionStatement {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroup>,
    pub function: Span,
    pub ampersand: Option<Span>,
    pub name: SimpleIdentifier,
    pub parameters: FunctionParameterList,
    pub return_type: Option<ReturnType>,
    pub body: FunctionBody,
}

impl Node for FunctionStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> =
            vec![&mut self.name, &mut self.parameters, &mut self.body];
        if let Some(return_type) = &mut self.return_type {
            children.push(return_type);
        }
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ClosureUseVariable {
    pub comments: CommentGroup,
    pub ampersand: Option<Span>,
    pub variable: SimpleVariable,
}

impl Node for ClosureUseVariable {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.variable]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ClosureUse {
    pub comments: CommentGroup,
    pub r#use: Span,
    pub left_parenthesis: Span,
    pub variables: CommaSeparated<ClosureUseVariable>,
    pub right_parenthesis: Span,
}

impl Node for ClosureUse {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.variables.children()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ClosureExpression {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroup>,
    pub r#static: Option<Span>,
    pub function: Span,
    pub ampersand: Option<Span>,
    pub parameters: FunctionParameterList,
    pub uses: Option<ClosureUse>,
    pub return_type: Option<ReturnType>,
    pub body: FunctionBody,
}

impl Node for ClosureExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.parameters];
        if let Some(uses) = &mut self.uses {
            children.push(uses);
        }
        if let Some(return_type) = &mut self.return_type {
            children.push(return_type);
        }
        children.push(&mut self.body);
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ArrowFunctionExpression {
    pub comments: CommentGroup,
    pub r#static: Option<Span>,
    pub ampersand: Option<Span>,
    pub r#fn: Span,
    pub attributes: Vec<AttributeGroup>,
    pub parameters: FunctionParameterList,
    pub return_type: Option<ReturnType>,
    pub double_arrow: Span,
    pub body: Box<Expression>,
}

impl Node for ArrowFunctionExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.parameters];
        if let Some(return_type) = &mut self.return_type {
            children.push(return_type);
        }
        children.push(self.body.as_mut());
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ConstructorParameter {
    pub attributes: Vec<AttributeGroup>,
    pub comments: CommentGroup,
    pub ampersand: Option<Span>,
    pub name: SimpleVariable,
    pub data_type: Option<Type>,
    pub ellipsis: Option<Span>,
    pub default: Option<Expression>,
    #[serde(flatten)]
    pub modifiers: PromotedPropertyModifierGroup,
}

impl Node for ConstructorParameter {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.name];
        if let Some(data_type) = &mut self.data_type {
            children.push(data_type);
        }
        if let Some(default) = &mut self.default {
            children.push(default);
        }
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ConstructorParameterList {
    pub comments: CommentGroup,
    pub left_parenthesis: Span,
    pub parameters: CommaSeparated<ConstructorParameter>,
    pub right_parenthesis: Span,
}

impl Node for ConstructorParameterList {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.parameters.children()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct AbstractConstructor {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroup>,
    #[serde(flatten)]
    pub modifiers: MethodModifierGroup,
    pub function: Span,
    // returning by reference from a constructor doesn't make sense
    // see: https://chat.stackoverflow.com/transcript/message/55718950#55718950
    pub ampersand: Option<Span>,
    pub name: SimpleIdentifier,
    pub parameters: FunctionParameterList,
    pub semicolon: Span,
}

impl Node for AbstractConstructor {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.name, &mut self.parameters]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ConcreteConstructor {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroup>,
    #[serde(flatten)]
    pub modifiers: MethodModifierGroup,
    pub function: Span,
    // returning by reference from a constructor doesn't make sense
    // see: https://chat.stackoverflow.com/transcript/message/55718950#55718950
    pub ampersand: Option<Span>,
    pub name: SimpleIdentifier,
    pub parameters: ConstructorParameterList,
    pub body: MethodBody,
}

impl Node for ConcreteConstructor {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.name, &mut self.parameters, &mut self.body]
    }
}

impl ConcreteConstructor {
    pub fn first_span(&self) -> Span {
        self.comments
            .comments
            .first()
            .map(|c| c.span)
            .unwrap_or_else(|| {
                self.attributes.first().map(|a| a.start).unwrap_or_else(|| {
                    self.modifiers
                        .modifiers
                        .first()
                        .map(|m| m.span())
                        .unwrap_or_else(|| self.function)
                })
            })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct AbstractMethod {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroup>,
    #[serde(flatten)]
    pub modifiers: MethodModifierGroup,
    pub function: Span,
    pub ampersand: Option<Span>,
    pub name: SimpleIdentifier,
    pub parameters: FunctionParameterList,
    pub return_type: Option<ReturnType>,
    pub semicolon: Span,
}

impl Node for AbstractMethod {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.name, &mut self.parameters];
        if let Some(return_type) = &mut self.return_type {
            children.push(return_type);
        }
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ConcreteMethod {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroup>,
    #[serde(flatten)]
    pub modifiers: MethodModifierGroup,
    pub function: Span,
    pub ampersand: Option<Span>,
    pub name: SimpleIdentifier,
    pub parameters: FunctionParameterList,
    pub return_type: Option<ReturnType>,
    pub body: MethodBody,
}

impl Node for ConcreteMethod {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.name, &mut self.parameters];
        if let Some(return_type) = &mut self.return_type {
            children.push(return_type);
        }
        children.push(&mut self.body);
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct MethodBody {
    pub comments: CommentGroup,
    pub left_brace: Span, // `{`
    pub statements: Vec<Statement>,
    pub right_brace: Span, // `}`
}

impl Node for MethodBody {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.statements
            .iter_mut()
            .map(|s| s as &mut dyn Node)
            .collect()
    }
}
