use std::path::PathBuf;

use pxp_parser::parse;
use pxp_symbol::SymbolTable;
use snappers::{snap, Snapper};

snap!(snapper, empty_file, process("fixtures/empty-file.php"));
snap!(snapper, tag, process("fixtures/tag.php"));
snap!(snapper, short_tag, process("fixtures/short-tag.php"));
snap!(snapper, echo_tag, process("fixtures/echo-tag.php"));
snap!(snapper, html, process("fixtures/html.php"));

fn snapper() -> Snapper {
    Snapper::new(
        format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "tests/__snapshots__").into()
    )
}

fn process(string_or_file: &str) -> String {
    let path = format!("{}/tests/{}", env!("CARGO_MANIFEST_DIR"), string_or_file);
    let path = PathBuf::from(path);
    let input = if path.exists() {
        std::fs::read(path).unwrap()
    } else {
        string_or_file.as_bytes().to_vec()
    };

    let mut symbol_table = SymbolTable::new();
    let result = parse(&input, &mut symbol_table);
    let mut output = format!("{:#?}\n---\n", result.ast);

    if !result.diagnostics.is_empty() {
        output.push_str(
            &result.diagnostics.iter().map(|d| d.to_string()).collect::<Vec<String>>().join("\n")
        );
    }

    output
}