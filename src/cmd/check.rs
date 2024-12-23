use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Perform static analysis on a file or directory.")]
pub struct Check {
    #[arg(help = "Paths to files or directories.")]
    path: Vec<PathBuf>,

    #[arg(short, long, help = "Only perform simple syntax checks.")]
    only_syntax: bool,
}

pub fn check(args: Check) -> anyhow::Result<()> {
    Ok(())
}
