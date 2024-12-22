use clap::Parser;

mod cmd;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser, Debug)]
enum Command {
    Tokenise(cmd::Tokenise)
}

fn main() -> anyhow::Result<()> {
    let parsed = Args::parse();

    match parsed.cmd {
        Command::Tokenise(args) => cmd::tokenise(args),
    }
}
