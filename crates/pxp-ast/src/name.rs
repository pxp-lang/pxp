use pxp_span::Span;
use pxp_symbol::Symbol;
use pxp_syntax::name::NameQualification;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Name {
    pub kind: NameKind,
    pub span: Span,
}

impl Name {
    pub fn new(kind: NameKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn missing(span: Span) -> Self {
        Self::new(NameKind::Resolved(ResolvedName { resolved: Symbol::missing(), original: Symbol::missing() }), span)
    }

    pub fn resolved(symbol: Symbol, original: Symbol, span: Span) -> Self {
        Self::new(NameKind::Resolved(ResolvedName { resolved: symbol, original }), span)
    }

    pub fn unresolved(symbol: Symbol, qualification: NameQualification, span: Span) -> Self {
        Self::new(NameKind::Unresolved(UnresolvedName { symbol, qualification }), span)
    }

    pub fn special(kind: SpecialNameKind, symbol: Symbol, span: Span) -> Self {
        Self::new(NameKind::Special(SpecialName { kind, symbol }), span)
    }

    pub fn symbol(&self) -> Symbol {
        match self.kind {
            NameKind::Special(s) => s.symbol,
            NameKind::Unresolved(u) => u.symbol,
            NameKind::Resolved(r) => r.resolved,
        }
    }

    pub fn is_special(&self) -> bool {
        matches!(self.kind, NameKind::Special(_))
    }

    pub fn is_unresolved(&self) -> bool {
        matches!(self.kind, NameKind::Unresolved(_))
    }

    pub fn is_resolved(&self) -> bool {
        matches!(self.kind, NameKind::Resolved(_))
    }
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