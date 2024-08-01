use std::collections::HashMap;

use pxp_ast::{Literal, *};
use pxp_index::Index;
use pxp_type::Type;
use pxp_ast::visitor::Visitor;

pub struct TypeMapGenerator;

impl TypeMapGenerator {
    pub fn generate(index: &Index, ast: &Vec<Statement>) -> TypeMap {
        let mut visitor = TypeMapVisitor { index, map: TypeMap::new() };

        visitor.visit(ast);
        visitor.map
    }
}

struct TypeMapVisitor<'a> {
    index: &'a Index,
    map: TypeMap,
}

impl<'a> Visitor for TypeMapVisitor<'a> {
    fn visit_literal(&mut self, node: &Literal) {
        self.map.insert(node.id, match &node.kind {
            LiteralKind::Integer => Type::Integer,
            LiteralKind::Float => Type::Float,
            LiteralKind::String => Type::String,
            LiteralKind::Missing => Type::Missing,
        });
    }
}

#[derive(Default, Debug)]
pub struct TypeMap {
    map: HashMap<NodeId, Type>,
}

impl TypeMap {
    pub fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, id: NodeId, ty: Type) {
        self.map.insert(id, ty);
    }

    pub fn get_type(&self, id: NodeId) -> &Type {
        self.map.get(&id).unwrap_or_else(|| &Type::Mixed)
    }
}