use crate::identifiers::SimpleIdentifier;

use crate::Statement;
use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct UnbracedNamespace {
    pub start: Span,                // `namespace`
    pub name: SimpleIdentifier,     // `Foo`
    pub end: Span,                  // `;`
    pub statements: Vec<Statement>, // `*statements*`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct BracedNamespace {
    pub namespace: Span,                // `namespace`
    pub name: Option<SimpleIdentifier>, // `Foo`
    pub body: BracedNamespaceBody,      // `{ *statements* }`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct BracedNamespaceBody {
    pub start: Span,                // `{`
    pub end: Span,                  // `}`
    pub statements: Vec<Statement>, // `*statements*`
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum NamespaceStatement {
    Unbraced(UnbracedNamespace), // `namespace Foo; *statements*`
    Braced(BracedNamespace),     // `namespace Foo { *statements* }`
}
