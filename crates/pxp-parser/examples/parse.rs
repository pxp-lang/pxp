use std::{env, process::exit};

use pxp_lexer::Lexer;
use pxp_parser::construct;
use pxp_source::{SourceFile, Language};

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();

    if args.is_empty() {
        eprintln!("Usage: parse <file>");
        exit(1);
    }

    let code = std::fs::read(&args[0]).unwrap();
    let lexer = Lexer::new();
    
    let source_file = SourceFile::new(args.get(0).cloned(), Language::Php, code);
    let tokens = match lexer.tokenize(&source_file) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{:#?}", err);
            exit(1);
        },
    };

    let ast = construct(&tokens);
    dbg!(ast);
}