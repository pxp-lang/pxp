use std::{
    env::args,
    path::PathBuf,
    process::exit,
    time::Instant,
};

use pxp_indexer::{Index, Indexer, write_index_to_cache, try_load_index_from_cache};
use pxp_symbol::SymbolTable;
use rustyline::{error::ReadlineError, DefaultEditor};

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let directory = PathBuf::from(args.get(0).unwrap());

    println!("Indexing...");
    let start = Instant::now();

    let mut indexer = if let Some(result) = try_load_index_from_cache(&directory) {
        Indexer::of(result.0, result.1)
    } else {
        Indexer::new()
    };

    let (index, symbol_table) = indexer.index(&directory);
    write_index_to_cache((&index, &symbol_table), &directory);

    let duration = start.elapsed();

    println!(
        "Indexing completed. Took {} milliseconds.",
        duration.as_millis()
    );
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
                    process_command(&command, &index, &symbol_table);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Exiting...");
                break;
            }
            _ => todo!(),
        }
    }
}

fn process_command(command: &str, index: &Index, symbol_table: &SymbolTable) {
    match command {
        "help" => {
            println!("help              Display this help message.");
            println!("search <name>     Search through the index for the given entity.");
            println!("dump              Output a minified list of all indexed entities.");
            println!("exit              Exit the REPL.")
        }
        "dump" => {
            print!("{:?}", index.debuggable(symbol_table));
        }
        "exit" => {
            exit(0);
        }
        _ => {
            println!("Unrecognised command.");
        }
    }
}
