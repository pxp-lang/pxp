use std::{env::args, path::Path, process::exit};

use discoverer::discover;
use pxp_lexer::Lexer;
use pxp_symbol::SymbolTable;
use pxp_token::Token;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("Usage: tokenise <path> --debug --immediate");
        exit(1);
    }

    let path = args.first().unwrap();
    let path = Path::new(path);
    let symbol_table = SymbolTable::the();
    let immediate = args.contains(&"--immediate".to_string());

    if path.is_dir() {
        let mut errors = Vec::new();
        let files = discover(&["php"], &[path.to_str().unwrap()]).unwrap();

        for file in files.iter() {
            if file.is_dir() {
                continue;
            }

            let contents = std::fs::read(file).unwrap();
            let mut lexer = Lexer::new(&contents[..], symbol_table);

            match if immediate { lexer.tokenize_in_immediate_mode() } else { lexer.tokenize() } {
                Ok(_) => {
                    print!(".");
                }
                Err(err) => {
                    errors.push((path.to_str().unwrap().to_string(), err));
                    print!("x");
                }
            }
        }

        println!();

        if errors.is_empty() {
            println!("Parsed directory with zero errors.");
        } else {
            for (path, error) in errors {
                println!("{}: {:?}", path, error);
            }
        }
    } else {
        let contents = std::fs::read(path).unwrap();
        let mut lexer = Lexer::new(&contents[..], symbol_table);
        let tokens = if immediate { lexer.tokenize_in_immediate_mode() } else { lexer.tokenize() }.unwrap();

        if args.contains(&"--debug".to_string()) {
            dbg_tokens(&tokens);
        }
    }
}

fn dbg_tokens(tokens: &[Token]) {
    for token in tokens.iter() {
        println!("{}", token.dbg());
    }
}
