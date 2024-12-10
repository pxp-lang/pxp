use std::env::args;

use pxp_lexer::Lexer;
use pxp_parser::Parser;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let file = args.first().unwrap();
    let contents = std::fs::read(file).unwrap();
    
    dbg!(Parser::parse(Lexer::new(&contents)));
}
