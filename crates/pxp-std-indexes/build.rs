use std::{path::PathBuf, process::Command};

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
        .spawn()
        .expect("Failed to generate stubs.");
}

fn generate_index_for_version(version: &str) {
    
}