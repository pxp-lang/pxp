use std::{env::var, path::PathBuf, process::Command};

use pxp_indexer::{serialize_index, Indexer};

const VERSIONS: [&str; 4] = ["8.0", "8.1", "8.2", "8.3"];

fn main() {
    clean_stubs_directory();
    generate_stubs();

    for version in VERSIONS {
        generate_index_for_version(version);
    }
}

fn clean_stubs_directory() {
    let stubs_directory = PathBuf::from(format!("{}/stubs", env!("CARGO_MANIFEST_DIR")));
    
    if !stubs_directory.exists() {
        return;
    }

    std::fs::remove_dir_all(&stubs_directory).unwrap();
}

fn generate_stubs() {
    Command::new("php")
        .arg("./separate-stubs-into-versions.php")
        .current_dir(format!("{}/meta", env!("CARGO_MANIFEST_DIR")))
        .status()
        .expect("Failed to generate stubs.");
}

fn generate_index_for_version(version: &str) {
    let stubs_directory = PathBuf::from(format!("{}/stubs/{}", env!("CARGO_MANIFEST_DIR"), version));
    let mut indexer = Indexer::new();

    let (index, symbol_table) = indexer.index(&stubs_directory);
    let serialized_index = serialize_index((&index, &symbol_table));

    std::fs::write(format!("{}/{}.index", var("OUT_DIR").unwrap(), version), serialized_index).unwrap();
}