use std::fmt::{Debug, Display};

use pxp_symbol::{Symbol, SymbolTable};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]

pub enum Type {
    Named(Symbol),
    Nullable(Box<Type>),
    Union(Vec<Type>),
    Intersection(Vec<Type>),
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
    GenericArray(Box<Type>, Box<Type>),
    EmptyArray,
    Object,
    Mixed,
    Callable,
    Iterable,
    StaticReference,
    SelfReference,
    ParentReference,
    Missing,
}

impl Default for Type {
    fn default() -> Self {
        Self::Missing
    }
}

impl Type {
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

    pub fn with_symbol_table<'a>(&self, symbol_table: &'a SymbolTable) -> TypeWithSymbolTable<'a> {
        TypeWithSymbolTable {
            r#type: self.clone(),
            symbol_table,
        }
    }
}

impl Display for Type {
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
            Type::GenericArray(key, value) => write!(f, "array<{}, {}>", key, value),
            Type::EmptyArray => write!(f, "empty-array"),
            Type::Object => write!(f, "object"),
            Type::Mixed => write!(f, "mixed"),
            Type::Callable => write!(f, "callable"),
            Type::Iterable => write!(f, "iterable"),
            Type::StaticReference => write!(f, "static"),
            Type::SelfReference => write!(f, "self"),
            Type::ParentReference => write!(f, "parent"),
            Type::Missing => write!(f, "<missing>"),
        }
    }
}

pub struct TypeWithSymbolTable<'a> {
    r#type: Type,
    symbol_table: &'a SymbolTable,
}

impl<'a> Debug for TypeWithSymbolTable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.r#type {
            Type::Named(name) => write!(f, "{}", self.symbol_table.resolve(*name).unwrap()),
            Type::Nullable(inner) => {
                write!(f, "?{:?}", inner.with_symbol_table(&self.symbol_table))
            }
            Type::Union(inner) => write!(
                f,
                "{}",
                inner
                    .iter()
                    .map(|t| format!("{:?}", t.with_symbol_table(&self.symbol_table)))
                    .collect::<Vec<String>>()
                    .join("|")
            ),
            Type::Intersection(inner) => write!(
                f,
                "{}",
                inner
                    .iter()
                    .map(|t| format!("{:?}", t.with_symbol_table(&self.symbol_table)))
                    .collect::<Vec<String>>()
                    .join("&")
            ),
            _ => write!(f, "{}", &self.r#type),
        }
    }
}
