use std::{env::args, path::PathBuf, io::{stdin, stdout, Write}, time::Instant, process::exit};

use pxp_indexer::{Indexer, Index};
use rustyline::{DefaultEditor, error::ReadlineError};

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let directory = PathBuf::from(args.get(0).unwrap());

    println!("Indexing...");
    let start = Instant::now();

    let mut indexer = Indexer::new();
    let index = indexer.index(vec![directory]);

    let duration = start.elapsed();

    println!("Indexing completed. Took {} milliseconds.", duration.as_millis());
    println!();
    println!("Enter a search query below to look through the index.");

    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let command = rl.readline(">> ");

        match &command {
            Ok(command) => {
                let _ = rl.add_history_entry(command);

                if command == "clear" {
                    rl.clear_screen().unwrap();
                } else {
                    process_command(&command, &index);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Exiting...");
                break;
            },
            _ => todo!()
        }
    }
}

fn process_command(command: &str, index: &Index) {
    match command {
        "help" => {
            println!("help              Display this help message.");
            println!("search <name>     Search through the index for the given entity.");
            println!("dump              Output a minified list of all indexed entities.");
            println!("exit              Exit the REPL.")
        },
        "dump" => {
            dbg!(index);
        },
        "exit" => {
            exit(0);
        },
        _ => {
            println!("Unrecognised command.");
        }
    }
}
