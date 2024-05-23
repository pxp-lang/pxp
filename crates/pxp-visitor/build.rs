use pxp_build::file_is_newer_than;

fn main() {
    println!("cargo:rerun-if-changed=../pxp-ast/meta/ast.yaml");

    let generated = ["src/visitor.rs", "src/visitor_mut.rs"];

    for file in &generated {
        if file_is_newer_than("../pxp-ast/meta/ast.yaml".into(), file.into()) {
            panic!("AST definition is newer than generated code. Please run `just generate-visitor` to update the generated code.");
        }
    }
}