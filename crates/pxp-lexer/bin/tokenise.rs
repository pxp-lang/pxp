use std::{env::args, process::exit, path::Path};

use discoverer::discover;
use pxp_lexer::Lexer;

const LEXER: Lexer = Lexer::new();

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("Usage: tokenise <path> [--debug]");
        exit(1);
    }

    let path = args.first().unwrap();
    let path = Path::new(path);

    if path.is_dir() {
        let mut errors = Vec::new();
        let files = discover(&["php"], &[path.to_str().unwrap()]).unwrap();

        for file in files.iter() {
            if file.is_dir() {
                continue;
            }

            let contents = std::fs::read(file).unwrap();
            match LEXER.tokenize(&contents[..]) {
                Ok(_) => {
                    print!(".");
                },
                Err(err) => {
                    errors.push((path.to_str().unwrap().to_string(), err));
                    print!("x");
                },
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
        let contents = std::fs::read(&path).unwrap();
        let tokens = LEXER.tokenize(&contents[..]).unwrap();
        if args.contains(&"--debug".to_string()) {
            dbg!(tokens);
        }
    }
}