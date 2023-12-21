use std::collections::HashMap;

use pxp_symbol::Symbol;

use crate::entities::FunctionEntity;

#[derive(Debug, Clone, Default)]
pub struct Index {
    // Using Symbol as the key for entities is a good idea because it
    // allows us to do super fast lookups when we have a resolved identifier.
    pub functions: HashMap<Symbol, FunctionEntity>,
}

impl Index {
    pub fn add_function(&mut self, function: FunctionEntity) {
        self.functions.insert(function.name, function);
    }
}
