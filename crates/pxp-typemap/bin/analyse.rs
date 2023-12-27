use std::{env::args, fs::read};

use pxp_ast::{FunctionCallExpression, ExpressionKind, identifiers::Identifier};
use pxp_parser::parse;
use pxp_symbol::SymbolTable;
use pxp_typemap::{TypeMapGenerator, TypeMap};
use pxp_visitor::{Visitor, walk_expression, walk_function_call};

/// An example program that uses the TypeMap component to analyse PHP code.
/// It doesn't do any type checking, but can be used to manually check the
/// output of a TypeMap using the `dumpType()` function, e.g. `dumpType($foo)`
/// will print the deduced / inferred type of the variable `$foo`.
fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let file = args.get(0).expect("Expected a file name");
    let code = read(&file).expect("Failed to read file");
    let mut symbol_table = SymbolTable::new();
    let mut result = parse(&code, &mut symbol_table);
    
    let mut type_map_generator = TypeMapGenerator::default();
    let type_map = type_map_generator.generate(&mut result.ast);

    let mut analyser = Analyser { type_map, symbol_table };
    analyser.visit(&mut result.ast);
}

struct Analyser {
    type_map: TypeMap,
    symbol_table: SymbolTable,
}

impl Visitor for Analyser {
    fn visit_function_call(&mut self, node: &mut FunctionCallExpression) {
        match &node.target.kind {
            ExpressionKind::Identifier(Identifier::SimpleIdentifier(identifier)) => match self.symbol_table.resolve(identifier.token.symbol.unwrap()) {
                Some(symbol) if symbol == b"dumpType" => {
                    println!("Line {}: {:?}", node.target.span.start.line, self.type_map.get_expr_type(node.arguments.arguments[0].get_value().id).unwrap().with_symbol_table(&self.symbol_table));
                },
                _ => walk_function_call(self, node)
            },
            _ => walk_function_call(self, node)
        }
    }
}