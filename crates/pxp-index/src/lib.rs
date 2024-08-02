use class_like::ClassLike;
use constant::Constant;
use function::Function;
use pxp_symbol::Symbol;
use std::collections::HashMap;

mod class_like;
mod constant;
mod function;
mod indexer;
mod parameter;
mod reflection;

pub use indexer::Indexer;
pub use reflection::*;

#[derive(Debug, Clone, Default)]
pub struct Index {
    classes: HashMap<Symbol, ClassLike>,
    functions: HashMap<Symbol, Function>,
    constants: HashMap<Symbol, Constant>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
            functions: HashMap::new(),
            constants: HashMap::new(),
        }
    }

    pub(crate) fn add_class(&mut self, class: ClassLike) {
        self.classes.insert(class.name, class);
    }

    pub(crate) fn add_function(&mut self, function: Function) {
        self.functions.insert(function.name, function);
    }

    pub(crate) fn add_constant(&mut self, constant: Constant) {
        self.constants.insert(constant.name, constant);
    }

    pub fn get_class(&self, name: Symbol) -> Option<ReflectionClass> {
        self.classes
            .get(&name)
            .map(|class| ReflectionClass { class, index: self })
    }

    pub fn get_function(&self, name: Symbol) -> Option<ReflectionFunction> {
        self.functions
            .get(&name)
            .map(|function| ReflectionFunction {
                function,
                index: self,
            })
    }

    pub fn get_constant(&self, name: Symbol) -> Option<ReflectionConstant> {
        self.constants
            .get(&name)
            .map(|constant| ReflectionConstant {
                constant,
                index: self,
            })
    }
}
