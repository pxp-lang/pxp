use pxp_bytestring::ByteString;
use pxp_span::Span;

#[derive(Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
    pub resolved: bool,
}

impl Type {
    pub fn is_resolved(&self) -> bool {
        self.resolved
    }
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Named(ByteString),
    Nullable(Box<Type>),
    Union(Box<Type>),
    Intersection(Box<Type>),
    Void,
    Null,
    True,
    False,
    String,
    Bool,
    Int,
    Float,
    Array,
    Object,
    Mixed,
    Callable,
    Iterable,
    StaticReference,
    SelfReference,
    ParentReference,
}