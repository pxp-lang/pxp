use std::path::{Path, PathBuf};

use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(version, about = "Initialise a new project.")]
pub struct Init {
    #[arg(short, long, help = "Overwrite an existing configuration file.")]
    force: bool,
}

const STUB: &str = include_str!("../stubs/pxp.config.toml");

pub fn init(args: Init) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;

    if cwd.join("pxp.config.toml").exists() && !args.force {
        anyhow::bail!(
            "Configuration file already exists. Use --force to overwrite existing configuration."
        );
    }

    let paths = find_interesting_directories_in(&cwd)?;
    let stub = STUB.replace(
        r#""<paths>""#,
        &paths
            .iter()
            .map(|p| format!(r#""{}""#, p.display()))
            .collect::<Vec<_>>()
            .join(",\n"),
    );

    std::fs::write(cwd.join("pxp.config.toml"), stub)?;

    println!(
        "{}",
        "Configuration file created.".green().bold().underline()
    );

    Ok(())
}

fn find_interesting_directories_in(path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    if path.join("src").exists() {
        paths.push(path.join("src").strip_prefix(path)?.to_path_buf());
    }

    if path.join("lib").exists() {
        paths.push(path.join("lib").strip_prefix(path)?.to_path_buf());
    }

    if path.join("artisan").exists() {
        paths.extend(vec![
            path.join("app").strip_prefix(path)?.to_path_buf(),
            path.join("bootstrap").strip_prefix(path)?.to_path_buf(),
            path.join("config").strip_prefix(path)?.to_path_buf(),
            path.join("database").strip_prefix(path)?.to_path_buf(),
            path.join("routes").strip_prefix(path)?.to_path_buf(),
            path.join("tests").strip_prefix(path)?.to_path_buf(),
        ]);
    }

    if path.join("tests").exists() {
        paths.push(path.join("tests").strip_prefix(path)?.to_path_buf());
    }

    paths.dedup();
    Ok(paths)
}
