use std::collections::HashMap;

use pxp_ast::{Name, NodeId};
use pxp_type::Type;

pub struct TypeMap {
    map: HashMap<NodeId, Type<Name>>,
}

/// A small wrapper around a dictionary that maps AST nodes to `Type<Name>` values based on their `NodeId`.
/// 
/// Using the `NodeId` allows you to generate the type information once for the given AST and
/// then use it across multiple passes without needing to regenerate or recalculate types.
impl TypeMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Insert a type for the given node.
    pub fn insert(&mut self, id: NodeId, ty: Type<Name>) {
        self.map.insert(id, ty);
    }

    /// Get the type for the given node.
    pub fn resolve(&self, id: NodeId) -> &Type<Name> {
        self.map.get(&id).unwrap_or_else(|| &Type::Mixed)
    }
}
