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
    #[clap(alias = "tokenize")]
    Tokenise(cmd::Tokenise),
    Parse(cmd::Parse),
    Check(cmd::Check),
}

fn main() -> anyhow::Result<()> {
    let parsed = Args::parse();

    match parsed.cmd {
        Command::Tokenise(args) => cmd::tokenise(args),
        Command::Parse(args) => cmd::parse(args),
        Command::Check(args) => cmd::check(args),
    }
}
