use std::{env::args, path::Path, process::exit};

use discoverer::discover;
use pxp_parser::parse;
use pxp_symbol::SymbolTable;

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
            let mut symbol_table = SymbolTable::new();
            parse(&contents, &mut symbol_table);
            print!(".");
        }

        println!();

        // if errors.is_empty() {
        println!("Parsed directory with zero errors.");
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
        let mut symbol_table = SymbolTable::new();
        let result = parse(&contents, &mut symbol_table);

        if args.contains(&"--debug".to_string()) {
            dbg!(result.ast);
        }

        if !result.diagnostics.is_empty() {
            for diagnostic in result.diagnostics.iter() {
                print!("{diagnostic}");
            }
        }
    }
}
