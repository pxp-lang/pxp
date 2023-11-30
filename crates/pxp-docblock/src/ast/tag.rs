use pxp_symbol::Symbol;
use pxp_type::Type;

use super::const_expr::ConstExpression;

#[derive(Debug, Clone)]
pub struct TagNode {
    pub tag: Tag,
    pub symbol: Symbol,
    pub description: Option<Symbol>,
}

#[derive(Debug, Clone)]
pub enum Tag {
    Param {
        name: Symbol,
        r#type: Type,
        reference: bool,
        variadic: bool,
    },
    Var {
        name: Option<Symbol>,
        r#type: Type,
    },
    Return {
        r#type: Type,
    },
    Throws {
        r#type: Type,
    },
    Mixin {
        r#type: Type,
    },
    Deprecated,
    Property {
        name: Symbol,
        r#type: Type,
    },
    Method {
        r#static: bool,
        return_type: Option<Type>,
        name: Symbol,
        // FIXME: Add template tags here.
        parameters: Vec<MethodParameter>,
    },
    Template {
        name: Symbol,
        bound: Option<Type>,
        default: Option<Type>,
    }
}

#[derive(Debug, Clone)]
pub struct MethodParameter {
    pub r#type: Type,
    pub reference: bool,
    pub variadic: bool,
    pub name: Symbol,
    pub default: ConstExpression,
}