use crate::{FunctionParameter, Block, SimpleVariable};

#[derive(Debug, Clone)]
pub struct ClosureExpression {
    // FIXME: Add comments here.
    // FIXME: Add attributes hre.
    pub is_static: bool,
    pub by_reference: bool,
    pub parameters: Vec<FunctionParameter>,
    pub uses: Option<ClosureUse>,
    // FIXME: Add return type here.
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct ClosureUse {
    pub variable: SimpleVariable,
    pub by_reference: bool,
}