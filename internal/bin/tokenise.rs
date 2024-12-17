use std::{env::args, path::Path, process::exit};

use discoverer::discover;
use pxp_lexer::Lexer;

use pxp_token::TokenKind;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("Usage: tokenise <path> --debug --immediate");
        exit(1);
    }

    let path = args.first().unwrap();
    let path = Path::new(path);
    let debug = args.contains(&"--debug".to_string());

    if path.is_dir() {
        let files = discover(&["php"], &[path.to_str().unwrap()]).unwrap();
        let mut count = 0;

        for file in files.iter() {
            if file.is_dir() {
                continue;
            }

            let contents = std::fs::read(file).unwrap();
            let mut lexer = Lexer::new(&contents[..]);

            loop {
                let token = lexer.current();

                if debug {
                    println!("{:?}: {:?}", token.kind, token.symbol);
                }

                if token.kind == TokenKind::Eof {
                    break;
                }

                lexer.next();
            }

            count += 1;
        }

        println!("{count} files tokenised");
    } else {
        let contents = std::fs::read(path).unwrap();
        let mut lexer = Lexer::new(&contents[..]);

        loop {
            let token = lexer.current();

            if debug {
                println!("{:?}: {:?}", token.kind, token.symbol);
            }

            if token.kind == TokenKind::Eof {
                break;
            }

            lexer.next();
        }
    }
}
