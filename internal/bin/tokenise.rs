use std::{env::args, path::Path, process::exit};

use discoverer::discover;
use pxp_lexer::Lexer;

use pxp_token::Token;
use pxp_span::Spanned;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("Usage: tokenise <path> --debug --immediate");
        exit(1);
    }

    let path = args.first().unwrap();
    let path = Path::new(path);
    let immediate = args.contains(&"--immediate".to_string());
    let no_output = args.contains(&"--no-output".to_string());

    if path.is_dir() {
        let mut errors = Vec::new();
        let files = discover(&["php"], &[path.to_str().unwrap()]).unwrap();
        let mut count = 0;

        for file in files.iter() {
            if file.is_dir() {
                continue;
            }

            let contents = std::fs::read(file).unwrap();
            let mut lexer = Lexer::new(&contents[..]);

            match if immediate {
                lexer.tokenize_in_immediate_mode()
            } else {
                lexer.tokenize()
            } {
                Ok(_) => {
                    if !no_output {
                        print!(".");
                    }
                }
                Err(err) => {
                    errors.push((path.to_str().unwrap().to_string(), err));

                    if !no_output {
                        print!("x");
                    }
                }
            }

            count += 1;
        }

        println!("{count} files tokenised");
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
        let mut lexer = Lexer::new(&contents[..]);
        let tokens = match if immediate {
            lexer.tokenize_in_immediate_mode()
        } else {
            lexer.tokenize()
        } {
            Ok(tokens) => tokens,
            Err(err) => {
                eprintln!("{}", err);

                let span = err.span();

                eprintln!("{}:{}", span.start_line(&contents), span.start_column(&contents));

                exit(1);
            }
        };

        if args.contains(&"--debug".to_string()) {
            dbg_tokens(&tokens);
        }
    }
}

fn dbg_tokens(tokens: &[Token]) {
    for token in tokens.iter() {
        println!("{:?}: {:?}", token.kind, token.symbol);
    }
}
