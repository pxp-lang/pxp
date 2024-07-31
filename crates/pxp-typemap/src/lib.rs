use std::collections::HashMap;

use pxp_ast::{NodeId, Statement};
use pxp_index::Index;
use pxp_type::Type;
use pxp_visitor::Visitor;

pub struct TypeMapGenerator;

impl TypeMapGenerator {
    pub fn generate(index: &Index, ast: &[Statement]) -> TypeMap {
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

}

#[derive(Default, Debug)]
pub struct TypeMap {
    map: HashMap<NodeId, Type>,
}

impl TypeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_type(&self, id: NodeId) -> &Type {
        self.map.get(&id).unwrap_or_else(|| &Type::Mixed)
    }
}