use pxp_span::Span;
use pxp_symbol::Symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstExpr {
    pub kind: ConstExprKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstExprKind {
    Null,
    True,
    False,
    Integer,
    Float,
    Array(Vec<ConstExprArrayItem>),
    ConstFetch(Symbol, Symbol),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstExprArrayItem {
    pub key: Option<ConstExpr>,
    pub value: ConstExpr,
}
