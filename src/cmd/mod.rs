use std::path::PathBuf;
use clap::{Parser, Subcommand};

mod init;
mod build;
mod analyse;

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
    #[clap(about = "Statically analyse a file or set of directories.")]
    Analyse(AnalyseCommand),
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

#[derive(Debug, Parser)]
pub struct AnalyseCommand {
    #[clap(help = "The path of a single file you would to build.")]
    file: Option<PathBuf>,
}

pub fn run() {
    let arguments = Arguments::parse();

    match arguments.command {
        Command::Init(command) => init::run(command),
        Command::Build(command) => build::run(command),
        Command::Analyse(command) => analyse::run(command),
    };
}