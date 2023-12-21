use std::{path::{PathBuf, Path}, fs::read};

use discoverer::discover;
use pxp_ast::functions::FunctionStatement;
use pxp_parser::parse;
use pxp_symbol::SymbolTable;
use pxp_visitor::{Visitor, walk_function};

use crate::{index::Index, FunctionEntity};

#[derive(Debug, Clone)]
pub struct Indexer {
    index: Index,
    symbol_table: SymbolTable,
}

impl Indexer {
    pub fn new() -> Self {
        Self {
            index: Index::default(),
            symbol_table: SymbolTable::default(),
        }
    }

    pub fn index(&mut self, directories: Vec<PathBuf>) -> Index {
        let files = discover(&["php"], &directories.iter().map(|d| d.to_str().unwrap()).collect::<Vec<&str>>()).unwrap();

        for file in files {
            self.index_file(file);
        }

        self.index.clone()
    }

    fn index_file(&mut self, file: PathBuf) {
        let contents = read(&file).unwrap();
        let mut program = parse(&contents, &mut self.symbol_table);

        self.visit(&mut program.ast);
    }

    pub fn of(index: Index, symbol_table: SymbolTable) -> Self {
        Self { index, symbol_table }
    }
}

impl Visitor for Indexer {
    fn visit_function(&mut self, node: &mut FunctionStatement) {
        let function = FunctionEntity::default();

        self.index.add_function(function);

        walk_function(self, node);
    }
}
