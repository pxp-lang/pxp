use std::env::args;

use discoverer::discover;
use indicatif::ProgressBar;
use pxp_bytestring::ByteString;
use pxp_index::{Index, Indexer};

use pxp_lexer::Lexer;
use pxp_parser::Parser;
use rustyline::DefaultEditor;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let directory = args.first().expect("error: no directory provided");
    let with_output = args.iter().any(|arg| arg == "--output");
    let files = discover(&["php"], &[directory]).unwrap();
    let mut index = Index::new();
    let mut indexer = Indexer::new(&mut index);

    if with_output {
        println!("Indexing...");
    }

    if with_output {
        let bar = ProgressBar::new(files.len() as u64);

        for file in files.iter() {
            let result = Parser::parse(Lexer::new(&std::fs::read(file).unwrap()));
            indexer.index(&result.ast);

            if with_output {
                bar.inc(1);
            }
        }

        let mut rl = DefaultEditor::new().unwrap();

        loop {
            let readline = rl.readline("search: ");

            match readline {
                Ok(line) => match *line.split_whitespace().collect::<Vec<_>>().as_slice() {
                    ["class", name] => {
                        let name = ByteString::from(name.as_bytes());
                        match index.get_class(&name) {
                            Some(class) => {
                                dbg!(class);
                            }
                            None => println!("class not found."),
                        }
                    }
                    ["classes"] => {
                        for class in index.get_classes() {
                            println!("{}", class.get_name());
                        }
                    }
                    _ => {
                        println!("unrecognised input.");
                    }
                },
                _ => panic!(),
            }
        }
    } else {
        for file in files.iter() {
            let result = Parser::parse(Lexer::new(&std::fs::read(file).unwrap()));
            indexer.index(&result.ast);
        }
    }
}
