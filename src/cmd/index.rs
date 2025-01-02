use std::path::{Path, PathBuf};

use clap::Parser;
use colored::Colorize;
use pxp_index::Index as Indexer;
use rustyline::{error::ReadlineError, CompletionType, Config, DefaultEditor, Editor};

use crate::utils::{find_php_files_in, pxp_home_dir, ProgressBar};

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Indexes a directory and provides a REPL for searching through the index."
)]
pub struct Index {
    #[clap(help = "The path to a file or directory.")]
    path: PathBuf,

    #[clap(short, long, help = "Do not show progress bar.")]
    no_progress: bool,
}

pub fn index(args: Index) -> anyhow::Result<()> {
    if !args.path.exists() {
        anyhow::bail!("The path `{}` does not exist.", args.path.display());
    }

    let mut index = Indexer::new();

    perform(&args, &mut index, &args.path)?;
    repl(&index)?;

    Ok(())
}

fn repl(index: &Indexer) -> anyhow::Result<()> {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .auto_add_history(true)
        .build();

    let mut rl = DefaultEditor::with_config(config)?;

    let history = pxp_home_dir()?.join(".index_history");
    if !history.exists() {
        std::fs::write(&history, "")?;
    }

    rl.load_history(&history)?;

    loop {
        let readline = rl.readline("index> ");

        match readline {
            Ok(command) => handle(&command, index)?,
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(_) => break,
        }
    }

    rl.append_history(&history)?;

    Ok(())
}

fn handle(command: &str, index: &Indexer) -> anyhow::Result<()> {
    let parts = command.split_whitespace().collect::<Vec<_>>();

    match &parts[..] {
        ["count", "functions"] => println!(
            "There are {} functions in the index.",
            index.number_of_functions().to_string().bold().underline()
        ),
        ["get", "function", name] => {
            let function = index.get_function(*name);

            match function {
                Some(function) => println!("{:#?}", function),
                None => println!("Function `{}` not found.", name.bold()),
            }
        }
        ["count", "classes"] => println!(
            "There are {} classes in the index.",
            index.number_of_classes().to_string().bold().underline()
        ),
        ["get", "class", name] => {
            let class = index.get_class(*name);

            match class {
                Some(class) => println!("{:#?}", class),
                None => println!("Class `{}` not found.", name.bold()),
            }
        }
        _ => println!("Unrecognised command: `{}`", command.red().bold()),
    }

    Ok(())
}

fn perform(args: &Index, index: &mut Indexer, path: &Path) -> anyhow::Result<()> {
    if path.is_file() {
        index.index_file(path);
    } else {
        let files = find_php_files_in(&path)?;
        let bar = ProgressBar::new(!args.no_progress, files.len() as u64);

        for file in files {
            bar.set_message(file.display().to_string());

            index.index_file(&file);

            bar.inc(1);
        }

        bar.finish_and_clear();
    }

    Ok(())
}
