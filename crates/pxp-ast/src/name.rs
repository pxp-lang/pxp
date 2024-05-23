use pxp_span::Span;
use pxp_symbol::Symbol;
use pxp_syntax::name::NameQualification;

use crate::{Name, NameKind, ResolvedName, SpecialName, SpecialNameKind, UnresolvedName};

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