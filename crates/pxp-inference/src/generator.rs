use pxp_ast::{*, visitor::Visitor};
use pxp_type::Type;

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
    fn visit_literal(&mut self, node: &Literal) {
        self.map.types.insert(node.id(), match node.kind {
            LiteralKind::String => Type::String,
            LiteralKind::Integer => Type::Integer,
            LiteralKind::Float => Type::Float,
            _ => Type::Mixed,
        });
    }

    fn visit_bool_expression(&mut self, node: &BoolExpression) {
        self.map.types.insert(node.id(), Type::Boolean);
    }
}