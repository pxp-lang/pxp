use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

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
        #[clap(short, long, help = "Print generated PHP code to stdout.")]
        stdout: bool,  
    },
}

fn main() {
    let arguments = Arguments::parse();

    env_logger::Builder::new()
        .filter_level(arguments.verbosity.log_level_filter())
        .init();

    match arguments.command {
        Command::Build { stdout } => {
            log::trace!("Starting build command...");
            log::info!("Build command set to print to stdout: {}", stdout);
        },
    };
}