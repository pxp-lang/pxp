use pxp_span::Span;
use pxp_symbol::Symbol;
use pxp_type::Type;

use self::const_expr::ConstExpr;

pub mod const_expr;

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Tag(Tag),
    Text(Text),
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub kind: TagKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum TagKind {
    Deprecated {
        description: Option<Symbol>,
    },
    Extends {
        r#type: Type,
        description: Option<Symbol>,
    },
    Generic {
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
        r#type: Type,
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
    }
}

#[derive(Debug, Clone)]
pub struct Template {
    pub name: Symbol,
    pub bound: Option<Type>,
    pub default: Option<Type>,
    pub description: Option<Symbol>,
}

#[derive(Debug, Clone)]
pub struct MethodTagParameter {
    pub r#type: Type,
    pub is_reference: bool,
    pub is_variadic: bool,
    pub name: Symbol,
    pub default: Option<ConstExpr>,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub text: Symbol,
}