use pxp_ast::{visitor::{walk_expression, Visitor}, *};
use pxp_index::Index;
use pxp_token::TokenKind;
use pxp_type::Type;

use crate::TypeMap;

/// The `TypeEngine` is responsible for generating a `TypeMap` for a given AST.
/// It uses the provided `Index` to resolve types for method calls, property accesses, etc.
pub struct TypeEngine<'a> {
    index: &'a Index
}

impl<'a> TypeEngine<'a> {
    /// Create a new `TypeEngine` with the provided `Index`.
    pub fn new(index: &'a Index) -> Self {
        TypeEngine { index }
    }

    /// Infer the types for the given AST and return a `TypeMap`.
    pub fn infer(&self, ast: &[Statement]) -> TypeMap {
        let mut map = TypeMap::new();
        
        let mut generator = TypeMapGenerator {
            map: &mut map,
            index: self.index
        };

        generator.visit(ast);
        map
    }
}

struct TypeMapGenerator<'a> {
    map: &'a mut TypeMap,
    index: &'a Index
}

impl<'a> Visitor for TypeMapGenerator<'a> {
    fn visit_expression(&mut self, node: &Expression) {
        walk_expression(self, node);

        let inner = self.map.resolve(node.kind.id()).clone();

        self.map.insert(node.id, inner);
    }

    fn visit_literal(&mut self, node: &Literal) {
        self.map.insert(node.id, match node.kind {
            LiteralKind::Integer => Type::Integer,
            LiteralKind::Float => Type::Float,
            LiteralKind::String => Type::String,
            LiteralKind::Missing => Type::Missing,
        })
    }

    fn visit_interpolated_string_expression(&mut self, node: &InterpolatedStringExpression) {
        self.map.insert(node.id, Type::String);
    }

    fn visit_bool_expression(&mut self, node: &BoolExpression) {
        self.map.insert(node.id, match node.value.kind {
            TokenKind::True => Type::True,
            TokenKind::False => Type::False,
            _ => Type::Boolean
        });
    }
}
