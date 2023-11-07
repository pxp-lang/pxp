use std::{env::args, process::exit};

use pxp_lexer::Lexer;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("Usage: tokenise <file>");
        exit(1);
    }

    let file = args.first().unwrap();
    let contents = std::fs::read(file).unwrap();

    let lexer = Lexer::new();
    let times = match args.iter().position(|a| a == "--times") {
        Some(i) => args[i + 1].parse::<usize>().unwrap(),
        None => 1
    };
    
    let with_output = args.contains(&"--output".to_string());

    for i in 0..times {
        if with_output {
            println!("Tokenising {} (no. {i})...", file);
        }

        let tokens = lexer.tokenize(&contents[..]).unwrap();

        if args.contains(&"--dump".to_string()) {
            dbg!(tokens);
        }
    }
}