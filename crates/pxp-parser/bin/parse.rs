use std::{env::args, process::exit, path::Path};

use pxp_parser::parse;
use walkdir::{WalkDir, DirEntry};
use std::io::Write;

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

        fn is_hidden(entry: &DirEntry) -> bool {
            entry.file_name()
                .to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false)
        }

        for entry in WalkDir::new(&path).into_iter().filter_entry(|e| !is_hidden(e)) {
            match entry {
                Ok(entry) => {
                    let path = entry.path();

                    if path.is_dir() {
                        continue;
                    }

                    if matches!(path.extension(), None) {
                        continue;
                    }

                    if path.extension().unwrap() != "php" {
                        continue;
                    }

                    // Purposefully skip this file because it has a known syntax error.
                    if path.ends_with("tests/Foundation/fixtures/bad-syntax-strategy.php") {
                        continue;
                    }

                    let contents = std::fs::read(path).unwrap();
                    match parse(&contents[..]) {
                        Ok(_) => {
                            print!(".");
                        },
                        Err(stack) => {
                            errors.push((path.to_str().unwrap().to_string(), stack.errors));
                            print!("x");
                        },
                    }
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
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