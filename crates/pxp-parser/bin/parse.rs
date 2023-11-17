use std::{env::args, process::exit, path::Path};

use discoverer::discover;
use pxp_parser::parse;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("Usage: parse <path> [--debug]");
        exit(1);
    }

    let path = args.first().unwrap();
    let path = Path::new(path);

    if path.is_dir() {
        let mut errors = Vec::new();
        let files = discover(&["php"], &[path.to_str().unwrap()]).unwrap();

        for file in files.iter() {
            // Purposefully skip this file because it has a known syntax error.
            if file.ends_with("tests/Foundation/fixtures/bad-syntax-strategy.php") {
                continue;
            }

            if file.is_dir() {
                continue;
            }

            let contents = std::fs::read(file).unwrap();
            match parse(&contents[..]) {
                Ok(_) => {
                    print!(".");
                },
                Err(stack) => {
                    errors.push((path.to_str().unwrap().to_string(), stack.errors));
                    print!("x");
                },
            }
        }

        println!();

        if errors.is_empty() {
            println!("Parsed directory with zero errors.");
        } else {
            println!("\nParsed directory with {} errors.", errors.len());
            for (path, errors) in errors {
                println!("{}:", path);
                for error in errors {
                    println!("  {}", error);
                }
            }
        }
    } else {
        let contents = std::fs::read(&path).unwrap();
        let ast = parse(&contents[..]).unwrap();
        if args.contains(&"--debug".to_string()) {
            dbg!(ast);
        }
    }
}