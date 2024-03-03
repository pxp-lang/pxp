use std::path::PathBuf;

fn main() {
    
}

fn stubs_directory() -> PathBuf {
    PathBuf::from(format!("{}/stubs", env!("CARGO_MANIFEST_DIR")))
}