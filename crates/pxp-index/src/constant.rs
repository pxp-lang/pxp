use pxp_symbol::Symbol;

#[derive(Debug, Clone)]
pub(crate) struct Constant {
    pub name: Symbol,
    pub short: Symbol,
    pub namespace: Option<Symbol>,
}