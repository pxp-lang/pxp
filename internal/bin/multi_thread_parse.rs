use std::{env::args, path::{Path, PathBuf}, process::exit, thread};

use discoverer::discover;
use pxp_parser::parse;


fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("Usage: multi-thread-parse <path>");
        exit(1);
    }

    let path = args.first().unwrap();
    let path = Path::new(path);

    if !path.is_dir() {
        eprintln!("the given path is not a directory.");
        exit(1);
    }

    let files = discover(&["php"], &[path.to_str().unwrap()]).unwrap();

    // Get the number of possible threads to use.
    let num_threads = num_cpus::get();

    // Split files into chunks and parse them in parallel.
    let chunks: Vec<Vec<PathBuf>> = files.chunks(files.len() / num_threads).map(|chunk| chunk.to_vec()).collect();
    let mut handles = Vec::new();

    for chunk in chunks {
        let handle = thread::spawn(move || {
            for file in chunk.iter() {
                // Purposefully skip this file because it has a known syntax error.
                if file.ends_with("tests/Foundation/fixtures/bad-syntax-strategy.php") {
                    continue;
                }

                if file.is_dir() {
                    continue;
                }

                let contents = std::fs::read(file).unwrap();
                parse(&contents);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Done!");
}
