use pxp_build::file_is_newer_than;

fn main() {
    println!("cargo:rerun-if-changed=meta/ast.yaml");

    if file_is_newer_than("meta/ast.yaml".into(), "src/generated.rs".into()) {
        panic!("AST definition is newer than generated code. Please run `just generate-ast` to update the generated code.");
    }
}
