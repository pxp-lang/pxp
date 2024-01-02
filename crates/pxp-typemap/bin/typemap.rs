use std::env::args;

use pxp_ast::{FunctionCallExpression, ExpressionKind, identifiers::{SimpleIdentifier, Identifier}};
use pxp_indexer::Indexer;
use pxp_parser::parse;
use pxp_symbol::SymbolTable;
use pxp_typemap::{TypeMapGenerator, TypeMap};
use pxp_visitor::{Visitor, walk_function_call};

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.len() != 1 {
        eprintln!("Usage: typemap <file>");
        return;
    }

    let file = args[0].clone();

    if ! file.ends_with("php") {
        eprintln!("Error: file must have .php extension");
        return;
    }

    if ! std::path::PathBuf::from(&file).exists() {
        eprintln!("Error: file does not exist");
        return;
    }

    let contents = std::fs::read(&file).unwrap();
    let directory = std::path::PathBuf::from(&file).parent().unwrap().to_path_buf();

    let mut symbol_table = SymbolTable::new();
    let mut parse_result = parse(&contents, &mut symbol_table);

    let mut indexer = Indexer::with_symbol_table(symbol_table.clone());
    let (index, _) = indexer.index(&directory);

    let mut type_map_generator = TypeMapGenerator::new(&index, &symbol_table);
    let type_map = type_map_generator.generate(&mut parse_result.ast);

    let mut type_dumper = TypeDumper {
        type_map: &type_map,
        symbol_table: &symbol_table,
    };

    type_dumper.visit(&mut parse_result.ast);
}

struct TypeDumper<'a> {
    type_map: &'a TypeMap,
    symbol_table: &'a SymbolTable,
}

impl<'a> Visitor for TypeDumper<'a> {
    fn visit_function_call(&mut self, node: &mut FunctionCallExpression) {
        if let ExpressionKind::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { token })) = &node.target.kind {
            if self.symbol_table.resolve(token.symbol.unwrap()).unwrap() == b"dumpType" {
                let argument = node.arguments.arguments.first().unwrap();
                
                if let Some(r#type) = self.type_map.get(argument.get_value().id) {
                    println!("{}: {}", argument.get_value().span.start.line, r#type);
                }
            }
        }

        walk_function_call(self, node);
    }
}