use std::path::{Path, PathBuf};

use clap::Parser as Args;
use pxp_diagnostics::DiagnosticKind;
use pxp_lexer::Lexer;
use pxp_parser::Parser;
use pxp_span::IsSpanned;

use crate::utils::find_php_files_in;

#[derive(Debug, Args)]
#[command(version, about = "Parse a file or directory.")]
pub struct Parse {
    #[arg(help = "The path to a file or directory.")]
    path: PathBuf,

    #[arg(short, long, help = "Dump the AST to stdout.")]
    dump: bool,

    #[arg(short = 'f', long, help = "Print filenames when parsing a directory.")]
    print_filenames: bool,

    #[arg(short, long, help = "Print diagnostics after parsing a file.")]
    print_diagnostics: bool,
}

pub fn parse(args: Parse) -> anyhow::Result<()> {
    let files = if args.path.is_dir() {
        find_php_files_in(&args.path)?
    } else {
        vec![args.path]
    };

    for file in files {
        if args.print_filenames {
            println!("{}", file.display());
        }

        parse_file(&file, args.dump, args.print_diagnostics)?;
    }

    Ok(())
}

fn parse_file(path: &Path, dump: bool, print_diagnostics: bool) -> anyhow::Result<()> {
    let contents = std::fs::read(path)?;
    let ast = Parser::parse(Lexer::new(&contents));

    if dump {
        println!("{:#?}", ast);
    }

    if print_diagnostics && !ast.diagnostics.is_empty() {
        for diagnostic in &ast.diagnostics {
            println!(
                "{} on line {}, column {}",
                diagnostic.kind.get_message(),
                diagnostic.span.start_line(&contents),
                diagnostic.span.start_column(&contents)
            );
        }
    }

    Ok(())
}
