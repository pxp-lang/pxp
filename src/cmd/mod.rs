use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod build;
mod init;

#[derive(Debug, Parser)]
#[command(version)]
struct Arguments {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(about = "Generate a configuration file.")]
    Init(InitCommand),
    #[clap(about = "Build a file or set of directories.")]
    Build(BuildCommand),
}

#[derive(Debug, Parser)]
pub struct InitCommand {
    #[clap(long, short, help = "Overwrite an existing configuration file.")]
    force: bool,
}

#[derive(Debug, Parser)]
pub struct BuildCommand {
    #[clap(help = "The path of a single file you would like to build.")]
    file: Option<PathBuf>,
}

pub fn run() {
    let arguments = Arguments::parse();

    match arguments.command {
        Command::Init(command) => init::run(command),
        Command::Build(command) => build::run(command),
    };
}
