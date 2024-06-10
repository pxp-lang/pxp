use std::collections::HashMap;
use class_like::ClassLike;
use function::Function;
use pxp_symbol::Symbol;

mod class_like;
mod parameter;
mod reflection;
mod indexer;
mod function;

pub use reflection::*;
pub use indexer::Indexer;

#[derive(Debug, Clone)]
pub struct Index {
    classes: HashMap<Symbol, ClassLike>,
    functions: HashMap<Symbol, Function>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub(crate) fn add_class(&mut self, class: ClassLike) {
        self.classes.insert(class.name, class);
    }

    pub(crate) fn add_function(&mut self, function: Function) {
        self.functions.insert(function.name, function);
    }

    pub fn get_class(&self, name: Symbol) -> Option<ReflectionClass> {
        self.classes.get(&name).map(|class| ReflectionClass { class, index: self })
    }

    pub fn get_function(&self, name: Symbol) -> Option<ReflectionFunction> {
        self.functions.get(&name).map(|function| ReflectionFunction { function, index: self })
    }
}