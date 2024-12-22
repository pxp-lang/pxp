use std::path::PathBuf;

use clap::Parser;
use pxp_lexer::Lexer;
use pxp_token::TokenKind;

use crate::utils::find_php_files_in;

#[derive(Parser, Debug)]
#[command(version, about = "Tokenise a file or directory.")]
pub struct Tokenise {
    #[arg(help = "The path to a file or directory.")]
    path: PathBuf,

    #[arg(short, long, help = "Dump the tokens to stdout.")]
    dump: bool,
}

pub fn tokenise(args: Tokenise) -> anyhow::Result<()> {
    let files = if args.path.is_dir() {
        find_php_files_in(&args.path)?
    } else {
        vec![args.path]
    };

    for file in files {
        tokenise_file(&file, args.dump)?;
    }

    Ok(())
}

fn tokenise_file(path: &PathBuf, dump: bool) -> anyhow::Result<()> {
    let contents = std::fs::read(path)?;
    let mut lexer = Lexer::new(&contents);

    loop {
        let current = lexer.current();

        if dump {
            println!("{:?} - {:?}", current.kind, current.symbol);
        }

        if current.kind == TokenKind::Eof {
            break;
        }

        lexer.next();
    }

    Ok(())
}
