use std::{env::args, path::Path, process::exit};

use discoverer::discover;
use pxp_lexer::Lexer;
use pxp_parser::Parser;
use pxp_span::Spanned;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("Usage: parse <path> [--debug]");
        exit(1);
    }

    let path = args.first().unwrap();
    let path = Path::new(path);

    if path.is_dir() {
        // let mut errors = Vec::new();
        let files = discover(&["php"], &[path.to_str().unwrap()]).unwrap();
        let print_filenames = args.contains(&"--print-filenames".to_string());
        let stop_on_diagnostics = args.contains(&"--stop-on-diagnostics".to_string());
        let mut count = 0;

        for file in files.iter() {
            // Purposefully skip this file because it has a known syntax error.
            if file.ends_with("tests/Foundation/fixtures/bad-syntax-strategy.php") {
                continue;
            }

            if file.is_dir() {
                continue;
            }

            if print_filenames {
                println!("{}", file.display());
            }

            let contents = std::fs::read(file).unwrap();
            let ast = Parser::parse(Lexer::new(&contents));

            if !ast.diagnostics.is_empty() {
                ast.diagnostics.iter().for_each(|error| {
                    println!("{:?}", error);
                });

                if stop_on_diagnostics {
                    break;
                }
            }

            count += 1;
        }

        println!("{count} files parsed");
        println!();

        // if errors.is_empty() {
        // println!("Parsed directory with zero errors.");
        // } else {
        // println!("\nParsed directory with {} errors.", errors.len());
        // for (path, errors) in errors {
        //     println!("{}:", path);
        //     for error in errors {
        //         println!("  {}", error);
        //     }
        // }
        // }
    } else {
        let contents = std::fs::read(path).unwrap();
        let result = Parser::parse(Lexer::new(&contents));

        if args.contains(&"--debug".to_string()) {
            dbg!(result.ast);
        }

        if !result.diagnostics.is_empty() {
            result.diagnostics.iter().for_each(|error| {
                println!("{:?}", error);
                println!(
                    "   line: {}, column: {}",
                    error.span.start_line(&contents) + 1,
                    error.span.start_column(&contents) + 1
                );
            });
        }
    }
}
