use std::{path::PathBuf, fs::write};
use colored::*;

use super::InitCommand;

pub fn run(args: InitCommand) {
    let stub = include_str!("../../stubs/pxp.toml");
    let path = PathBuf::from("./pxp.toml");

    if path.exists() && ! args.force {
        eprintln!("{} {}", "Error:".red().bold(), "configuration file already exists");
        return;
    }

    match write(path, stub) {
        Ok(_) => {
            println!("{} {}", "Success:".green().bold(), "configuration file created");
        },
        Err(error) => {
            eprintln!("{} {}", "Error:".red().bold(), error.to_string().red());
        },
    }
}