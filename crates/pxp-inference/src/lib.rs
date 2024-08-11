use std::collections::HashMap;

mod generator;

use pxp_ast::{Node, NodeId, Statement};
use pxp_index::Index;
use pxp_symbol::Symbol;
use pxp_type::Type;

/// The main type inference engine.
/// 
/// This uses an `Index` to perform type inference based on the various structures
/// that are discovered inside of a project or workspace.
#[derive(Debug, Clone, Copy)]
pub struct InferenceEngine<'i> {
    index: &'i Index,
}

impl<'i> InferenceEngine<'i> {
    pub fn new(index: &'i Index) -> Self {
        InferenceEngine { index }
    }

    /// Generate a `TypeMap` from the given AST.
    pub fn map(&self, ast: &[Statement]) -> TypeMap {
        let mut generator = generator::TypeMapGenerator::new(); 
        generator.generate(ast)
    }
}

/// A map of `NodeId` values to their associated types.
#[derive(Debug, Clone)]
pub struct TypeMap {
    types: HashMap<NodeId, Type<Symbol>>,
}

impl TypeMap {
    pub(crate) fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    pub fn resolve(&self, id: NodeId) -> &Type<Symbol> {
        self.types.get(&id).unwrap_or_else(|| &Type::Mixed)
    }
}