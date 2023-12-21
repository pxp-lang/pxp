use std::{path::{PathBuf, Path}, fs::read};

use discoverer::discover;
use pxp_ast::functions::FunctionStatement;
use pxp_parser::parse;
use pxp_symbol::{SymbolTable, Symbol};
use pxp_visitor::{Visitor, walk_function};

use crate::{index::Index, FunctionEntity};

#[derive(Debug, Clone)]
pub struct Indexer {
    index: Index,
    symbol_table: SymbolTable,
    scope: Scope,
}

#[derive(Debug, Clone, Default)]
struct Scope {
    namespace: Option<Symbol>,
}

impl Scope {
    pub fn namespace(&self) -> Option<&Symbol> {
        self.namespace.as_ref()
    }
}

impl Indexer {
    pub fn new() -> Self {
        Self {
            index: Index::default(),
            symbol_table: SymbolTable::default(),
            scope: Scope::default(),
        }
    }

    pub fn index(&mut self, directories: Vec<PathBuf>) -> (Index, SymbolTable) {
        let files = discover(&["php"], &directories.iter().map(|d| d.to_str().unwrap()).collect::<Vec<&str>>()).unwrap();

        for file in files {
            self.index_file(file);
        }

        (self.index.clone(), self.symbol_table.clone())
    }

    fn index_file(&mut self, file: PathBuf) {
        let contents = read(&file).unwrap();
        let mut program = parse(&contents, &mut self.symbol_table);

        self.visit(&mut program.ast);
    }

    fn qualify(&mut self, symbol: Symbol) -> Symbol {
        if let Some(namespace) = self.scope.namespace() {

        } else {
            symbol
        }
    }

    pub fn of(index: Index, symbol_table: SymbolTable) -> Self {
        Self { index, symbol_table, scope: Scope::default() }
    }
}

impl Visitor for Indexer {
    fn visit_function(&mut self, node: &mut FunctionStatement) {
        let mut function = FunctionEntity::default();

        function.short_name = node.name.token.symbol.unwrap();

        self.index.add_function(function);

        walk_function(self, node);
    }
}
