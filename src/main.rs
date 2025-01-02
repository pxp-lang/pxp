use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};

mod cmd;
mod config;
mod utils;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().bold())
    .usage(AnsiColor::Green.on_default().bold())
    .literal(AnsiColor::Blue.on_default().underline())
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, styles = STYLES)]
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
    Init(cmd::Init),
    Index(cmd::Index),
}

fn main() -> anyhow::Result<()> {
    let parsed = Args::parse();

    match parsed.cmd {
        Command::Tokenise(args) => cmd::tokenise(args),
        Command::Parse(args) => cmd::parse(args),
        Command::Check(args) => cmd::check(args),
        Command::Init(args) => cmd::init(args),
        Command::Index(args) => cmd::index(args),
    }
}
