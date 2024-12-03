use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash, Default)]

pub enum Type<N: Debug + Display> {
    Named(N),
    Nullable(Box<Type<N>>),
    Union(Vec<Type<N>>),
    Intersection(Vec<Type<N>>),
    Void,
    Null,
    True,
    False,
    Never,
    Float,
    Boolean,
    Integer,
    String,
    Array,
    Object,
    #[default]
    Mixed,
    Callable,
    Iterable,
    StaticReference,
    SelfReference,
    ParentReference,
    TypedArray(Box<Type<N>>, Box<Type<N>>),
    Missing,
}

impl<N: Debug + Display> Type<N> {
    pub fn map<T: Debug + Display>(&self, cb: impl FnOnce(&Type<N>) -> Type<T>) -> Type<T> {
        cb(self)
    }

    pub fn standalone(&self) -> bool {
        matches!(
            self,
            Type::Mixed | Type::Never | Type::Void | Type::Nullable(_)
        )
    }

    pub fn nullable(&self) -> bool {
        matches!(self, Type::Nullable(_))
    }

    pub fn includes_callable(&self) -> bool {
        match &self {
            Self::Callable => true,
            Self::Union(types) | Self::Intersection(types) => {
                types.iter().any(|x| x.includes_callable())
            }
            _ => false,
        }
    }

    pub fn is_bottom(&self) -> bool {
        matches!(self, Type::Never | Type::Void)
    }

    pub fn is_object_like(&self) -> bool {
        match self {
            Type::Named(_) | Type::Object => true,
            Type::Nullable(inner) => inner.is_object_like(),
            Type::Union(inner) => inner.iter().any(|t| t.is_object_like()),
            Type::Intersection(inner) => inner.iter().any(|t| t.is_object_like()),
            // FIXME: Add static/self/parent here.
            _ => false,
        }
    }

    pub fn array_key_types() -> Type<N> {
        Self::Union(vec![Self::String, Self::Integer])
    }
}

impl<N: Debug + Display> Display for Type<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Type::Named(inner) => write!(f, "{}", inner),
            Type::Nullable(inner) => write!(f, "?{}", inner),
            Type::Union(inner) => write!(
                f,
                "{}",
                inner
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(" | ")
            ),
            Type::Intersection(inner) => write!(
                f,
                "{}",
                inner
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(" & ")
            ),
            Type::Void => write!(f, "void"),
            Type::Null => write!(f, "null"),
            Type::True => write!(f, "true"),
            Type::False => write!(f, "false"),
            Type::Never => write!(f, "never"),
            Type::Float => write!(f, "float"),
            Type::Boolean => write!(f, "bool"),
            Type::Integer => write!(f, "int"),
            Type::String => write!(f, "string"),
            Type::Array => write!(f, "array"),
            Type::Object => write!(f, "object"),
            Type::Mixed => write!(f, "mixed"),
            Type::Callable => write!(f, "callable"),
            Type::Iterable => write!(f, "iterable"),
            Type::StaticReference => write!(f, "static"),
            Type::SelfReference => write!(f, "self"),
            Type::ParentReference => write!(f, "parent"),
            Type::TypedArray(key, value) => write!(f, "array<{}, {}>", key, value),
            Type::Missing => write!(f, "<missing>"),
        }
    }
}
