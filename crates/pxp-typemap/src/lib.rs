use std::collections::HashMap;

use pxp_ast::{NodeId, Statement};
use pxp_index::Index;
use pxp_type::Type;

pub struct TypeMapGenerator;

impl TypeMapGenerator {
    pub fn generate(index: &Index, ast: &[Statement]) {

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
}