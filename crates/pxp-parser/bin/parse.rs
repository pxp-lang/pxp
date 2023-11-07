use std::{env::args, process::exit};

use pxp_parser::parse;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("Usage: parse <file> [--debug]");
        exit(1);
    }

    let file = args.first().unwrap();
    let contents = std::fs::read(file).unwrap();

    let ast = parse(&contents[..]).unwrap();

    if args.contains(&"--debug".to_string()) {
        dbg!(ast);
    }
}