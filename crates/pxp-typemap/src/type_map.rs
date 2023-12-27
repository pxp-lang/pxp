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

    pub fn insert_expr_type(&mut self, node_id: NodeId, ty: Type) {
        self.exprs.insert(node_id, ty);
    }

    pub fn get_expr_type(&self, node_id: NodeId) -> Option<&Type> {
        self.exprs.get(&node_id)
    }
}