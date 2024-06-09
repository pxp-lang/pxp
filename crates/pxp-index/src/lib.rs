use std::collections::HashMap;
use class_like::ClassLike;
use pxp_symbol::Symbol;

mod class_like;
mod parameter;
mod reflection;
mod indexer;

pub use reflection::*;
pub use indexer::Indexer;

#[derive(Debug, Clone)]
pub struct Index {
    classes: HashMap<Symbol, ClassLike>
}

impl Index {
    pub fn new() -> Self {
        Self {
            classes: HashMap::new()
        }
    }

    pub(crate) fn add_class(&mut self, class: ClassLike) {
        self.classes.insert(class.name, class);
    }

    pub fn get_class(&self, name: Symbol) -> Option<ReflectionClass> {
        self.classes.get(&name).map(|class| ReflectionClass { class, index: self })
    }
}