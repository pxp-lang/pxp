use pxp_bytestring::ByteString;
use pxp_span::Span;

#[derive(Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
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
    /// This is a special type that indicates the underlying type has been resolved,
    /// resulting in a fully qualified name for union/intersection/named types.
    Resolved(Box<Type>),
}

impl TypeKind {
    pub fn is_resolved(&self) -> bool {
        match self {
            TypeKind::Resolved(_) => true,
            _ => false,
        }
    }
}