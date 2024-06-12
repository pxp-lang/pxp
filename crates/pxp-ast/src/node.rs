use std::any::{Any, TypeId};

use pxp_span::Span;

use crate::utils::CommaSeparated;

pub trait Node: Any {
    fn name(&self) -> &'static str;

    fn children(&self) -> Vec<&dyn Node> {
        Vec::new()
    }

    fn span(&self) -> Span {
        Span::default()
    }
}

pub fn downcast<T: Node + 'static>(node: &dyn Node) -> Option<&T> {
    let t = TypeId::of::<T>();
    let n = node.type_id();

    if t == n {
        unsafe { Some(&*(node as *const dyn Node as *const T)) }
    } else {
        None
    }
}

impl<T: Node> Node for Vec<T> {
    fn name(&self) -> &'static str {
        "Vec"
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.iter().map(|n| n as &dyn Node).collect()
    }

    fn span(&self) -> Span {
        if let Some(first) = self.first() {
            if let Some(last) = self.last() {
                Span::new(first.span().start, last.span().end)
            } else {
                first.span()
            }
        } else {
            Span::default()
        }
    }
}

impl<T: Node> Node for Option<T> {
    fn name(&self) -> &'static str {
        "Option"
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            Some(n) => vec![n as &dyn Node],
            None => Vec::new(),
        }
    }

    fn span(&self) -> Span {
        if let Some(n) = self {
            n.span()
        } else {
            Span::default()
        }
    }
}

impl<T: Node> Node for Box<T> {
    fn name(&self) -> &'static str {
        "Box"
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![self.as_ref() as &dyn Node]
    }

    fn span(&self) -> Span {
        self.as_ref().span()
    }
}

impl<T: Node> Node for CommaSeparated<T> {
    fn name(&self) -> &'static str {
        "CommaSeparated"
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.inner.iter().map(|n| n as &dyn Node).collect()
    }

    fn span(&self) -> Span {
        if let Some(first) = self.inner.first() {
            if let Some(last) = self.inner.last() {
                Span::new(first.span().start, last.span().end)
            } else {
                first.span()
            }
        } else {
            Span::default()
        }
    }
}
