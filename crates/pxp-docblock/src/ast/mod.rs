use std::fmt::{self, Debug, Formatter};

use pxp_span::Span;
use pxp_symbol::{Symbol, SymbolTable};
use pxp_type::Type;

use self::const_expr::ConstExpr;

pub mod const_expr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub kind: NodeKind,
    pub span: Span,
}

impl Node {
    pub fn new(kind: NodeKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn with_symbol_table<'a>(&self, symbol_table: &'a SymbolTable) -> NodeWithSymbolTable<'a> {
        NodeWithSymbolTable {
            node: self.clone(),
            symbol_table,
        }
    }
}

pub struct NodeWithSymbolTable<'a> {
    node: Node,
    symbol_table: &'a SymbolTable,
}

impl<'a> Debug for NodeWithSymbolTable<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.node.kind {
            NodeKind::Text(text) => {
                write!(
                    f,
                    "Text({:?})",
                    self.symbol_table.resolve(text.text).unwrap()
                )
            }
            NodeKind::Tag(tag) => tag.with_symbol_table(self.symbol_table).fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Tag(Tag),
    Text(Text),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    pub kind: TagKind,
    pub span: Span,
}

impl Tag {
    pub fn new(kind: TagKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl Tag {
    fn with_symbol_table<'a>(&self, symbol_table: &'a SymbolTable) -> TagWithSymbolTable<'a> {
        TagWithSymbolTable {
            tag: self.clone(),
            symbol_table,
        }
    }
}

struct TagWithSymbolTable<'a> {
    tag: Tag,
    symbol_table: &'a SymbolTable,
}

impl<'a> Debug for TagWithSymbolTable<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.tag.kind {
            TagKind::Generic { tag, description } => match description {
                Some(description) => write!(f, "Tag::Generic\n\ttag: {}\n\tdescription: {}", self.symbol_table.resolve(*tag).unwrap(), self.symbol_table.resolve(*description).unwrap()),
                None => write!(f, "Tag::Generic\n\ttag: {}", self.symbol_table.resolve(*tag).unwrap()),
            },
            TagKind::Param { r#type, name, is_reference, is_variadic, description } => {
                match r#type {
                    Some(r#type) => match description {
                        Some(description) => write!(f, "Tag::Param\n\ttype: {:?}\n\tname: {}\n\tis_reference: {}\n\tis_variadic: {}\n\tdescription: {}", r#type.with_symbol_table(self.symbol_table), self.symbol_table.resolve(*name).unwrap(), is_reference, is_variadic, self.symbol_table.resolve(*description).unwrap()),
                        None => write!(f, "Tag::Param\n\ttype: {:?}\n\tname: {}\n\tis_reference: {}\n\tis_variadic: {}", r#type.with_symbol_table(self.symbol_table), self.symbol_table.resolve(*name).unwrap(), is_reference, is_variadic),
                    },
                    None => match description {
                        Some(description) => write!(f, "Tag::Param\n\tname: {}\n\tis_reference: {}\n\tis_variadic: {}\n\tdescription: {}", self.symbol_table.resolve(*name).unwrap(), is_reference, is_variadic, self.symbol_table.resolve(*description).unwrap()),
                        None => write!(f, "Tag::Param\n\tname: {}\n\tis_reference: {}\n\tis_variadic: {}", self.symbol_table.resolve(*name).unwrap(), is_reference, is_variadic),
                    }
                }
            },
            _ => self.tag.fmt(f)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagKind {
    Deprecated {
        description: Option<Symbol>,
    },
    Extends {
        r#type: Type,
        description: Option<Symbol>,
    },
    Generic {
        tag: Symbol,
        description: Option<Symbol>,
    },
    Implements {
        r#type: Type,
        description: Option<Symbol>,
    },
    Method {
        name: Symbol,
        return_type: Type,
        is_static: bool,
        templates: Vec<Template>,
        parameters: Vec<MethodTagParameter>,
        description: Option<Symbol>,
    },
    Mixin {
        r#type: Type,
        description: Option<Symbol>,
    },
    ParamOut {
        r#type: Type,
        name: Symbol,
        description: Option<Symbol>,
    },
    Param {
        r#type: Option<Type>,
        name: Symbol,
        is_reference: bool,
        is_variadic: bool,
        description: Option<Symbol>,
    },
    Property {
        r#type: Type,
        name: Symbol,
        description: Option<Symbol>,
    },
    Return {
        r#type: Type,
        description: Option<Symbol>,
    },
    SelfOut {
        r#type: Type,
        description: Option<Symbol>,
    },
    Template(Template),
    Throws {
        r#type: Type,
        description: Option<Symbol>,
    },
    Uses {
        r#type: Type,
        description: Option<Symbol>,
    },
    Var {
        r#type: Type,
        name: Option<Symbol>,
        description: Option<Symbol>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    pub name: Symbol,
    pub bound: Option<Type>,
    pub default: Option<Type>,
    pub description: Option<Symbol>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodTagParameter {
    pub r#type: Type,
    pub is_reference: bool,
    pub is_variadic: bool,
    pub name: Symbol,
    pub default: Option<ConstExpr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    pub text: Symbol,
}

impl Text {
    pub fn new(text: Symbol) -> Self {
        Self { text }
    }
}
