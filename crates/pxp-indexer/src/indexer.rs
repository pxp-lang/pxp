use std::{path::{PathBuf, Path}, fs::read};

use discoverer::discover;
use pxp_ast::{functions::FunctionStatement, namespaces::{UnbracedNamespace, BracedNamespace}};
use pxp_parser::parse;
use pxp_span::Span;
use pxp_symbol::{SymbolTable, Symbol};
use pxp_type::Type;
use pxp_visitor::{Visitor, walk_function, walk_braced_namespace, walk_unbraced_namespace};

use crate::{index::Index, FunctionEntity, ParameterEntity, Location};

#[derive(Debug, Clone)]
pub struct Indexer {
    index: Index,
    symbol_table: SymbolTable,
    scope: Scope,
}

#[derive(Debug, Clone, Default)]
struct Scope {
    file: String,
    namespace: Option<Symbol>,
}

impl Scope {
    pub fn namespace(&self) -> Option<&Symbol> {
        self.namespace.as_ref()
    }

    pub fn file(&self) -> &str {
        &self.file
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

        self.scope.file = file.to_str().unwrap().to_string();
        self.visit(&mut program.ast);
    }

    fn qualify(&mut self, symbol: Symbol) -> Symbol {
        if let Some(namespace) = self.scope.namespace() {
            self.symbol_table.coagulate(&[*namespace, symbol], Some(b"\\"))
        } else {
            symbol
        }
    }

    pub fn of(index: Index, symbol_table: SymbolTable) -> Self {
        Self { index, symbol_table, scope: Scope::default() }
    }
}

impl Visitor for Indexer {
    fn visit_unbraced_namespace(&mut self, node: &mut UnbracedNamespace) {
        self.scope.namespace = Some(node.name.token.symbol.unwrap());
        walk_unbraced_namespace(self, node);
        self.scope.namespace = None;
    }

    fn visit_braced_namespace(&mut self, node: &mut BracedNamespace) {
        if let Some(name) = &node.name {
            self.scope.namespace = Some(name.token.symbol.unwrap());
        }

        walk_braced_namespace(self, node);

        if node.name.is_some() {
            self.scope.namespace = None;
        }
    }

    fn visit_function(&mut self, node: &mut FunctionStatement) {
        let mut function = FunctionEntity::default();

        let short_name = node.name.token.symbol.unwrap();
        function.name = self.qualify(short_name);
        function.short_name = short_name;

        let mut parameters = Vec::new();

        for parameter in node.parameters.iter() {
            let mut p = ParameterEntity::default();
            p.name = parameter.name.token.symbol.unwrap();
            p.reference = parameter.ampersand.is_some();
            p.variadic = parameter.ellipsis.is_some();
            p.optional = parameter.default.is_some();
            p.r#type = if let Some(r#type) = &parameter.data_type {
                r#type.clone()
            } else {
                Type::Mixed(Span::default())
            };

            parameters.push(p);
        }

        function.parameters = parameters;
        function.return_type = if let Some(return_type) = &node.return_type {
            return_type.data_type.clone()
        } else {
            Type::Mixed(Span::default())
        };

        function.location = Location::new(self.scope.file().to_string(), Span::new(node.name.token.span.start, node.body.right_brace.end));

        self.index.add_function(function);

        walk_function(self, node);
    }
}
