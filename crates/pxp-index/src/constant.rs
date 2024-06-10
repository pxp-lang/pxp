use pxp_symbol::Symbol;

#[derive(Debug, Clone)]
pub struct Constant {
    pub(crate) name: Symbol,
    pub(crate) short: Symbol,
    pub(crate) namespace: Option<Symbol>,
}