use std::path::Path;

use entities::{EntityRegistry, FunctionEntity};
use file::FileRegistry;

mod file;
mod entities;
mod location;
mod indexer;

pub use file::{FileId, HasFileId};
use indexer::IndexingVisitor;
use pxp_ast::visitor::Visitor;
use pxp_bytestring::ByteString;
use pxp_lexer::Lexer;
use pxp_parser::Parser;

#[derive(Debug, Clone)]
pub struct Index {
    files: FileRegistry,
    pub(crate) entities: EntityRegistry,
}

impl Index {
    pub fn new() -> Self {
        Self {
            files: FileRegistry::new(),
            entities: EntityRegistry::new(),
        }
    }

    pub fn index_file(&mut self, path: &Path) {
        let file_id = self.files.get_or_insert(path);
        let contents = std::fs::read(path).unwrap();
        let parse_result = Parser::parse(Lexer::new(&contents));
        
        let mut visitor = IndexingVisitor::new(file_id, self);
        visitor.visit(&parse_result.ast);
    }

    pub fn number_of_files(&self) -> usize {
        self.files.len()
    }

    pub fn number_of_functions(&self) -> usize {
        self.entities.functions().len()
    }

    pub fn get_function(&self, name: impl Into<ByteString>) -> Option<&FunctionEntity> {
        self.entities.get_function_by_name(name)
    }

    pub fn get_file_path(&self, from: impl HasFileId) -> Option<&std::path::Path> {
        self.files.get_file_path(from.file_id())
    }

    pub fn get_file_path_unchecked(&self, from: impl HasFileId) -> &std::path::Path {
        self.files.get_file_path_unchecked(from.file_id())
    }
}