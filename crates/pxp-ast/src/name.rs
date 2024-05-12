use pxp_span::Span;
use pxp_symbol::Symbol;
use pxp_syntax::name::NameQualification;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Name {
    pub kind: NameKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum NameKind {
    Special(SpecialName),
    Unresolved(UnresolvedName),
    Resolved(ResolvedName),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct SpecialName {
    pub kind: SpecialNameKind,
    pub symbol: Symbol,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum SpecialNameKind {
    Self_,
    Static,
    Parent,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct UnresolvedName {
    pub symbol: Symbol,
    pub qualification: NameQualification,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct ResolvedName {
    pub resolved: Symbol,
    pub original: Symbol,
}