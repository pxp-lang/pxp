use std::{any::{Any, TypeId}, fmt::Debug};

use pxp_span::Spanned;

use crate::utils::CommaSeparated;

pub trait Node: Any + Spanned + Debug {
    fn name(&self) -> &'static str;

    fn children(&self) -> Vec<&dyn Node> {
        Vec::new()
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
}

impl<T: Node> Node for Box<T> {
    fn name(&self) -> &'static str {
        "Box"
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![self.as_ref() as &dyn Node]
    }
}

impl<T: Node> Node for CommaSeparated<T> {
    fn name(&self) -> &'static str {
        "CommaSeparated"
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.inner.iter().map(|n| n as &dyn Node).collect()
    }
}
