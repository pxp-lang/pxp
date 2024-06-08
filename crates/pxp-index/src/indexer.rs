use pxp_symbol::SymbolTable;
use pxp_visitor::Visitor;

use crate::Index;

#[derive(Debug, Clone)]
pub struct Indexer {
    index: Index,
    symbol_table: SymbolTable,
}

impl Visitor for Indexer {
    
}