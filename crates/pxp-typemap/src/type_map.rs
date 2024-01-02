use std::collections::HashMap;

use pxp_ast::NodeId;
use pxp_type::Type;

#[derive(Debug, Clone, Default)]
pub struct TypeMap {
    exprs: HashMap<NodeId, Type>,
}

impl TypeMap {
    pub fn new() -> Self {
        Self {
            exprs: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: NodeId, ty: Type) {
        self.exprs.insert(id, ty);
    }

    pub fn get(&self, id: NodeId) -> Option<&Type> {
        self.exprs.get(&id)
    }
}