use std::env::args;

use pxp_lexer::Lexer;
use pxp_token::TokenKind;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let file = args.first().unwrap();
    let contents = std::fs::read(file).unwrap();
    let mut lexer = Lexer::new(&contents);

    loop {
        let token = lexer.current();

        println!("{:?} -> {:?}", token.kind, token.symbol);

        if token.kind == TokenKind::Eof {
            break;
        }

        lexer.next();
    }
}
