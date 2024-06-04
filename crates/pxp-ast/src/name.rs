use std::fmt::{Debug, Display};

use pxp_span::Span;
use pxp_symbol::Symbol;
use pxp_syntax::name::NameQualification;

use crate::{Name, NameKind, ResolvedName, SpecialName, SpecialNameKind, UnresolvedName};

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            NameKind::Special(s) => write!(f, "{}", s.symbol),
            NameKind::Unresolved(u) => write!(f, "{}", u.symbol),
            NameKind::Resolved(r) => write!(f, "{}", r.resolved),
        }
    }
}

impl Name {
    pub fn new(kind: NameKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn missing(span: Span) -> Self {
        Self::new(
            NameKind::Resolved(ResolvedName {
                resolved: Symbol::missing(),
                original: Symbol::missing(),
            }),
            span,
        )
    }

    pub fn resolved(symbol: Symbol, original: Symbol, span: Span) -> Self {
        Self::new(
            NameKind::Resolved(ResolvedName {
                resolved: symbol,
                original,
            }),
            span,
        )
    }

    pub fn unresolved(symbol: Symbol, qualification: NameQualification, span: Span) -> Self {
        Self::new(
            NameKind::Unresolved(UnresolvedName {
                symbol,
                qualification,
            }),
            span,
        )
    }

    pub fn special(kind: SpecialNameKind, symbol: Symbol, span: Span) -> Self {
        Self::new(NameKind::Special(SpecialName { kind, symbol }), span)
    }

    pub fn symbol(&self) -> Symbol {
        match &self.kind {
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

impl Display for SpecialName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            SpecialNameKind::Self_ => write!(f, "self"),
            SpecialNameKind::Static => write!(f, "static"),
            SpecialNameKind::Parent => write!(f, "parent"),
        }
    }
}

impl Display for UnresolvedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

impl Display for ResolvedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.original)
    }
}
