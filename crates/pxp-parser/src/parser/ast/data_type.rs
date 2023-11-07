use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use std::fmt::Display;

use crate::lexer::byte_string::ByteString;
use crate::lexer::token::Span;
use crate::node::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Type {
    Named(Span, ByteString),
    Nullable(Span, Box<Type>),
    Union(Vec<Type>),
    Intersection(Vec<Type>),
    Void(Span),
    Null(Span),
    True(Span),
    False(Span),
    Never(Span),
    Float(Span),
    Boolean(Span),
    Integer(Span),
    String(Span),
    Array(Span),
    Object(Span),
    Mixed(Span),
    Callable(Span),
    Iterable(Span),
    StaticReference(Span),
    SelfReference(Span),
    ParentReference(Span),
}

impl Type {
    pub fn standalone(&self) -> bool {
        matches!(
            self,
            Type::Mixed(_) | Type::Never(_) | Type::Void(_) | Type::Nullable(_, _)
        )
    }

    pub fn nullable(&self) -> bool {
        matches!(self, Type::Nullable(_, _))
    }

    pub fn includes_callable(&self) -> bool {
        match &self {
            Self::Callable(_) => true,
            Self::Union(types) | Self::Intersection(types) => {
                types.iter().any(|x| x.includes_callable())
            }
            _ => false,
        }
    }

    pub fn includes_class_scoped(&self) -> bool {
        match &self {
            Self::StaticReference(_) | Self::SelfReference(_) | Self::ParentReference(_) => true,
            Self::Union(types) | Self::Intersection(types) => {
                types.iter().any(|x| x.includes_class_scoped())
            }
            _ => false,
        }
    }

    pub fn is_bottom(&self) -> bool {
        matches!(self, Type::Never(_) | Type::Void(_))
    }

    pub fn first_span(&self) -> Span {
        match &self {
            Type::Named(span, _) => *span,
            Type::Nullable(span, _) => *span,
            Type::Union(inner) => inner[0].first_span(),
            Type::Intersection(inner) => inner[0].first_span(),
            Type::Void(span) => *span,
            Type::Null(span) => *span,
            Type::True(span) => *span,
            Type::False(span) => *span,
            Type::Never(span) => *span,
            Type::Float(span) => *span,
            Type::Boolean(span) => *span,
            Type::Integer(span) => *span,
            Type::String(span) => *span,
            Type::Array(span) => *span,
            Type::Object(span) => *span,
            Type::Mixed(span) => *span,
            Type::Callable(span) => *span,
            Type::Iterable(span) => *span,
            Type::StaticReference(span) => *span,
            Type::SelfReference(span) => *span,
            Type::ParentReference(span) => *span,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Type::Named(_, inner) => write!(f, "{}", inner),
            Type::Nullable(_, inner) => write!(f, "?{}", inner),
            Type::Union(inner) => write!(
                f,
                "{}",
                inner
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join("|")
            ),
            Type::Intersection(inner) => write!(
                f,
                "{}",
                inner
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join("&")
            ),
            Type::Void(_) => write!(f, "void"),
            Type::Null(_) => write!(f, "null"),
            Type::True(_) => write!(f, "true"),
            Type::False(_) => write!(f, "false"),
            Type::Never(_) => write!(f, "never"),
            Type::Float(_) => write!(f, "float"),
            Type::Boolean(_) => write!(f, "bool"),
            Type::Integer(_) => write!(f, "int"),
            Type::String(_) => write!(f, "string"),
            Type::Array(_) => write!(f, "array"),
            Type::Object(_) => write!(f, "object"),
            Type::Mixed(_) => write!(f, "mixed"),
            Type::Callable(_) => write!(f, "callable"),
            Type::Iterable(_) => write!(f, "iterable"),
            Type::StaticReference(_) => write!(f, "static"),
            Type::SelfReference(_) => write!(f, "self"),
            Type::ParentReference(_) => write!(f, "parent"),
        }
    }
}

impl Node for Type {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            Type::Nullable(_, t) => vec![t.as_mut() as &mut dyn Node],
            Type::Union(ts) => ts.iter_mut().map(|x| x as &mut dyn Node).collect(),
            Type::Intersection(ts) => ts.iter_mut().map(|x| x as &mut dyn Node).collect(),
            _ => vec![],
        }
    }
}
