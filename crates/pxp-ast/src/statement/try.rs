use pxp_type::Type;

use crate::{Block, SimpleVariable};

#[derive(Debug, Clone)]
pub struct TryStatement {
    pub body: Block,
    pub catches: Vec<Catch>,
    pub finally: Option<Finally>,
}

#[derive(Debug, Clone)]
pub struct Catch {
    pub r#type: Type,
    pub variable: Option<SimpleVariable>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Finally {
    pub body: Block,
}