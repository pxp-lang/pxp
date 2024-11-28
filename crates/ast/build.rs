use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=meta/ast.yaml");

    if file_is_newer_than("meta/ast.yaml".into(), "src/generated.rs".into()) {
        panic!("AST definition is newer than generated code. Please run `just generate-ast` to update the generated code.");
    }
}

fn file_is_newer_than(file: PathBuf, other: PathBuf) -> bool {
    let file_metadata = std::fs::metadata(file).unwrap();
    let other_metadata = std::fs::metadata(other).unwrap();

    file_metadata.modified().unwrap() > other_metadata.modified().unwrap()
}
