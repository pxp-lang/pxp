use std::slice::Iter;

use crate::attributes::AttributeGroup;
use crate::data_type::Type;
use crate::identifiers::SimpleIdentifier;
use crate::modifiers::MethodModifierGroup;
use crate::modifiers::PromotedPropertyModifierGroup;

use crate::utils::CommaSeparated;
use crate::variables::SimpleVariable;
use crate::Expression;
use crate::Statement;

use pxp_span::Span;
use pxp_syntax::comments::CommentGroup;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ReturnType {
    pub colon: Span,
    pub data_type: Type,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct FunctionParameter {
    pub comments: CommentGroup,
    pub name: SimpleVariable,
    pub attributes: Vec<AttributeGroup>,
    pub data_type: Option<Type>,
    pub ellipsis: Option<Span>,
    pub default: Option<Expression>,
    pub ampersand: Option<Span>,
}

#[derive(Debug, PartialEq, Eq, Clone)]

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

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct FunctionBody {
    pub comments: CommentGroup,
    pub left_brace: Span,
    pub statements: Vec<Statement>,
    pub right_brace: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]

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

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ClosureUseVariable {
    pub comments: CommentGroup,
    pub ampersand: Option<Span>,
    pub variable: SimpleVariable,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ClosureUse {
    pub comments: CommentGroup,
    pub r#use: Span,
    pub left_parenthesis: Span,
    pub variables: CommaSeparated<ClosureUseVariable>,
    pub right_parenthesis: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]

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

#[derive(Debug, PartialEq, Eq, Clone)]

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

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ConstructorParameter {
    pub attributes: Vec<AttributeGroup>,
    pub comments: CommentGroup,
    pub ampersand: Option<Span>,
    pub name: SimpleVariable,
    pub data_type: Option<Type>,
    pub ellipsis: Option<Span>,
    pub default: Option<Expression>,

    pub modifiers: PromotedPropertyModifierGroup,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ConstructorParameterList {
    pub comments: CommentGroup,
    pub left_parenthesis: Span,
    pub parameters: CommaSeparated<ConstructorParameter>,
    pub right_parenthesis: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct AbstractConstructor {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroup>,

    pub modifiers: MethodModifierGroup,
    pub function: Span,
    // returning by reference from a constructor doesn't make sense
    // see: https://chat.stackoverflow.com/transcript/message/55718950#55718950
    pub ampersand: Option<Span>,
    pub name: SimpleIdentifier,
    pub parameters: FunctionParameterList,
    pub semicolon: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ConcreteConstructor {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroup>,

    pub modifiers: MethodModifierGroup,
    pub function: Span,
    // returning by reference from a constructor doesn't make sense
    // see: https://chat.stackoverflow.com/transcript/message/55718950#55718950
    pub ampersand: Option<Span>,
    pub name: SimpleIdentifier,
    pub parameters: ConstructorParameterList,
    pub body: MethodBody,
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

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct AbstractMethod {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroup>,

    pub modifiers: MethodModifierGroup,
    pub function: Span,
    pub ampersand: Option<Span>,
    pub name: SimpleIdentifier,
    pub parameters: FunctionParameterList,
    pub return_type: Option<ReturnType>,
    pub semicolon: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ConcreteMethod {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroup>,

    pub modifiers: MethodModifierGroup,
    pub function: Span,
    pub ampersand: Option<Span>,
    pub name: SimpleIdentifier,
    pub parameters: FunctionParameterList,
    pub return_type: Option<ReturnType>,
    pub body: MethodBody,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct MethodBody {
    pub comments: CommentGroup,
    pub left_brace: Span, // `{`
    pub statements: Vec<Statement>,
    pub right_brace: Span, // `}`
}
