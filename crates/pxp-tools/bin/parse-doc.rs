use std::env::args;

use pxp_docblock::{lexer::Lexer, parser::Parser};
use pxp_symbol::SymbolTable;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let file = args.first().unwrap();
    let content = std::fs::read(file).unwrap();

    let mut symbol_table = SymbolTable::the();
    let mut lexer = Lexer::new(&mut symbol_table);

    let tokens = lexer.tokenize(&content).unwrap();

    if args.contains(&"--dump-tokens".to_string()) {
        println!("Tokens:");

        for token in tokens.iter() {
            println!("{:?}", token.with_symbol_table(&symbol_table));
        }
    }

    let parser = Parser::new();
    let ast = parser.parse(&tokens, &mut symbol_table);

    if args.contains(&"--dump-ast".to_string()) {
        println!("AST:");

        for node in ast.unwrap().iter() {
            println!("{:?}", node.with_symbol_table(&symbol_table));
        }
    }
}
