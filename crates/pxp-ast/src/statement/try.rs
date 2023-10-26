use crate::{Block, SimpleVariable};

#[derive(Debug, Clone)]
pub struct TryStatement {
    pub body: Block,
    pub catches: Vec<Catch>,
    pub finally: Option<Finally>,
}

#[derive(Debug, Clone)]
pub struct Catch {
    // FIXME: Add type here.
    pub variable: Option<SimpleVariable>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Finally {
    pub body: Block,
}