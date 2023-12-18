use std::env::args;

use pxp_docblock::lexer::Lexer;
use pxp_symbol::SymbolTable;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let file = args.first().unwrap();
    let content = std::fs::read(file).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut lexer = Lexer::new(&mut symbol_table);

    let tokens = lexer.tokenize(&content).unwrap();

    if args.contains(&"--dump-tokens".to_string()) {
        println!("Tokens:");
        
        for token in tokens.iter() {
            println!("{:?}", token.with_symbol_table(&symbol_table));
        }
    }
}