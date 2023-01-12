use clap::{Parser, Subcommand};

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
    Init(InitCommand)
}

#[derive(Debug, Parser)]
pub struct InitCommand {
    #[clap(long, short, help = "Overwrite an existing configuration file.")]
    force: bool,
}

pub fn run() {
    let arguments = Arguments::parse();

    match arguments.command {
        Command::Init(command) => init::run(command),
    };
}