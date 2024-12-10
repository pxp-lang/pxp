use std::{env::args, path::{Path, PathBuf}};

use discoverer::discover;
use pxp_lexer::Lexer;
use pxp_token::TokenKind;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let input = args.first().unwrap();
    let path = PathBuf::from(&input);
    let debug = args.iter().any(|arg| arg == "--debug");

    if path.is_file() {
        tokenise_file(&path, debug);
        return;
    }

    let files = discover(&["php"], &[input]).unwrap();

    for file in files.iter() {
        tokenise_file(file, debug);
    }
}

fn tokenise_file(file: &Path, debug: bool) {
    let contents = std::fs::read(file).unwrap();
    let mut lexer = Lexer::new(&contents);

    loop {
        let token = lexer.current();

        if debug {
            println!("{:?} -> {:?}", token.kind, token.symbol);
        }

        if token.kind == TokenKind::Eof {
            break;
        }

        lexer.next();
    }
}
