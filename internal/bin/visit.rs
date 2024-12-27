use std::{env::args, path::Path, process::exit};

use discoverer::discover;
use pxp_ast::visitor::Visitor;
use pxp_lexer::Lexer;
use pxp_parser::Parser;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("Usage: visit <path>");
        exit(1);
    }

    let path = args.first().unwrap();
    let path = Path::new(path);

    if path.is_dir() {
        // let mut errors = Vec::new();
        let files = discover(&["php"], &[path.to_str().unwrap()]).unwrap();
        let print_filenames = args.contains(&"--print-filenames".to_string());
        let stop_on_errors = args.contains(&"--stop-on-errors".to_string());

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
            let ast = Parser::parse(Lexer::new(&contents), None);

            if !ast.diagnostics.is_empty() && stop_on_errors {
                ast.diagnostics.iter().for_each(|error| {
                    println!("{}", error);
                });

                break;
            }

            let mut visitor = NullVisitor;
            visitor.visit(&ast.ast);
        }
    }
}

struct NullVisitor;

impl Visitor for NullVisitor {}
