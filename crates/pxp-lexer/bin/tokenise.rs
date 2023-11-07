use std::{env::args, process::exit};

use pxp_lexer::Lexer;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("Usage: tokenise <file> [--debug, --times <n>]");
        exit(1);
    }

    let file = args.first().unwrap();
    let contents = std::fs::read(file).unwrap();

    let lexer = Lexer::new();
    let times = match args.iter().position(|a| a == "--times") {
        Some(i) => args[i + 1].parse::<usize>().unwrap(),
        None => 1
    };

    let debug = args.contains(&"--debug".to_string());

    for i in 0..times {
        if debug {
            println!("Tokenising {} (no. {i})...", file);
        }

        let tokens = lexer.tokenize(&contents[..]).unwrap();

        if debug {
            dbg!(tokens);
        }
    }
}