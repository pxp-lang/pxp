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
/// 
/// The main usage of the type inference engine is inside of the static analyser and
/// language server. That has directly impacted the design of this crate.
/// 
/// Generating a `TypeMap` is relatively low-cost, so when you need to provide a completion (for example),
/// you can take the latest version of the AST, get the `TypeMap` and then for the same AST, get the
/// type of `Node` that you are interested in. Using the same AST will ensure that the `NodeId` values
/// are the same, making lookups cheap.
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