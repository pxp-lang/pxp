use pxp_ast::{visitor::Visitor, Statement};

use crate::TypeMap;

/// An internal set of methods for generating a `TypeMap` from an AST.
/// 
/// This is used internally by the `InferenceEngine` to generate a `TypeMap` from an AST.
pub(super) struct TypeMapGenerator {
    map: TypeMap,
}

impl TypeMapGenerator {
    pub fn new() -> Self {
        TypeMapGenerator {
            map: TypeMap::new(),
        }
    }

    pub fn generate(&mut self, ast: &[Statement]) -> TypeMap {
        // FIXME: The `Visitor` trait itself needs to accept a slice
        //        rather than a reference to a `Vec<Statement>`.
        let ast = ast.to_vec();

        self.visit(&ast);

        self.map.clone()
    }
}

/// Handles traversing the AST and generating a `TypeMap`.
impl Visitor for TypeMapGenerator {

}