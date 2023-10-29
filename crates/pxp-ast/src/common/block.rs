use crate::Statement;

#[derive(Debug, Clone, Default)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            statements: Vec::with_capacity(8),
        }
    }

    pub fn push(&mut self, statement: Statement) {
        self.statements.push(statement);
    }

    pub fn to_vec(&self) -> Vec<Statement> {
        self.statements.clone()
    }
}

impl From<Vec<Statement>> for Block {
    fn from(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone)]
pub enum StatementOrBlock {
    Statement(Box<Statement>),
    Block(Block),
}

impl StatementOrBlock {
    pub fn is_statement(&self) -> bool {
        match self {
            StatementOrBlock::Statement(_) => true,
            StatementOrBlock::Block(_) => false,
        }
    }

    pub fn is_block(&self) -> bool {
        match self {
            StatementOrBlock::Statement(_) => false,
            StatementOrBlock::Block(_) => true,
        }
    }
}

impl From<Statement> for StatementOrBlock {
    fn from(statement: Statement) -> Self {
        Self::Statement(Box::new(statement))
    }
}

impl From<Block> for StatementOrBlock {
    fn from(block: Block) -> Self {
        Self::Block(block)
    }
}

impl From<Vec<Statement>> for StatementOrBlock {
    fn from(statements: Vec<Statement>) -> Self {
        Self::Block(statements.into())
    }
}