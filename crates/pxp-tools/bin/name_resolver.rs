use std::{env::args, fs::read};

use pxp_name_resolver::NameResolvingVisitor;
use pxp_parser::parse;
use pxp_symbol::SymbolTable;
use pxp_visitor::VisitorMut;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let file = args.get(0).expect("File not provided.");
    let contents = read(file).expect("Failed to read file.");
    let mut result = parse(&contents, SymbolTable::the());
    let mut name_resolver = NameResolvingVisitor::new(SymbolTable::the());
    
    name_resolver.visit(&mut result.ast);

    if args.contains(&"--context".to_string()) {
        dbg!(name_resolver.get_context());
    }
    
    if args.contains(&"--ast".to_string()) {
        dbg!(result.ast);
    }
}