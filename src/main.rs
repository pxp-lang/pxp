use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use cmd::BuildOptions;

mod cmd;
mod config;
mod printer;
mod transpile;
mod visitors;

#[derive(Debug, Parser)]
struct Arguments {
    #[clap(subcommand)]
    command: Command,

    #[clap(flatten)]
    verbosity: Verbosity,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(name = "build")]
    Build {
        #[clap(help = "Path to file to build.")]
        file: Option<PathBuf>,

        #[clap(short, long, help = "Print generated PHP code to stdout.")]
        stdout: bool,
    },

    #[clap(name = "init")]
    Init {
        #[clap(short, long, help = "Overwrite an existing pxp.toml file.")]
        force: bool,
    },
}

fn main() {
    let arguments = Arguments::parse();

    env_logger::Builder::new()
        .filter_level(arguments.verbosity.log_level_filter())
        .init();

    match arguments.command {
        Command::Build { file, stdout } => {
            let options = BuildOptions { stdout };

            if let Some(file) = file {
                cmd::build_single_file(file, options);
            } else {
                cmd::build(options);
            }
        },
        Command::Init { force } => {
            cmd::init(force);
        },
    };
}
