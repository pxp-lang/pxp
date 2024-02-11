use std::path::PathBuf;

use pxp_parser::parse;
use pxp_symbol::SymbolTable;
use snappers::{snap, Snapper};

// Tags & Inline HTML
snap!(snapper, empty_file, process("fixtures/empty-file.php"));
snap!(snapper, tag, process("fixtures/tag.php"));
snap!(snapper, short_tag, process("fixtures/short-tag.php"));
snap!(snapper, echo_tag, process("fixtures/echo-tag.php"));
snap!(snapper, html, process("fixtures/html.php"));

// Echo
snap!(snapper, simple_echo, process("fixtures/simple-echo.php"));
snap!(snapper, multi_echo, process("fixtures/multi-echo.php"));
snap!(snapper, echo_no_value, process("fixtures/echo-no-value.php"));
snap!(snapper, echo_single_value_trailing_comma, process("fixtures/echo-single-value-trailing-comma.php"));
snap!(snapper, echo_missing_semicolon, process("fixtures/echo-missing-semicolon.php"));

// Assignments
snap!(snapper, assign, process("fixtures/assign.php"));
snap!(snapper, multi_assign, process("fixtures/multi-assign.php"));
snap!(snapper, add_assign, process("fixtures/add-assign.php"));
snap!(snapper, sub_assign, process("fixtures/sub-assign.php"));
snap!(snapper, mul_assign, process("fixtures/mul-assign.php"));
snap!(snapper, div_assign, process("fixtures/div-assign.php"));
snap!(snapper, mod_assign, process("fixtures/mod-assign.php"));
snap!(snapper, exp_assign, process("fixtures/exp-assign.php"));
snap!(snapper, concat_assign, process("fixtures/concat-assign.php"));
snap!(snapper, bitwise_and_assign, process("fixtures/bitwise-and-assign.php"));
snap!(snapper, bitwise_or_assign, process("fixtures/bitwise-or-assign.php"));
snap!(snapper, bitwise_xor_assign, process("fixtures/bitwise-xor-assign.php"));
snap!(snapper, bitwise_left_shift_assign, process("fixtures/bitwise-left-shift-assign.php"));
snap!(snapper, bitwise_right_shift_assign, process("fixtures/bitwise-right-shift-assign.php"));
snap!(snapper, coalesce_assign, process("fixtures/coalesce-assign.php"));

// Arithmetic
snap!(snapper, add, process("fixtures/add.php"));
snap!(snapper, sub, process("fixtures/sub.php"));
snap!(snapper, mul, process("fixtures/mul.php"));
snap!(snapper, div, process("fixtures/div.php"));
snap!(snapper, mod_, process("fixtures/mod.php"));
snap!(snapper, exp, process("fixtures/exp.php"));
snap!(snapper, post_inc, process("fixtures/post-inc.php"));
snap!(snapper, post_dec, process("fixtures/post-dec.php"));
snap!(snapper, pre_inc, process("fixtures/pre-inc.php"));
snap!(snapper, pre_dec, process("fixtures/pre-dec.php"));

// Bitwise

// Comparison

// Logical

// Literals
snap!(snapper, int, process("fixtures/int.php"));
snap!(snapper, float, process("fixtures/float.php"));
snap!(snapper, string, process("fixtures/string.php"));
snap!(snapper, null, process("fixtures/null.php"));
snap!(snapper, bool, process("fixtures/bool.php"));
snap!(snapper, empty_array, process("fixtures/empty-array.php"));
snap!(snapper, single_item_array, process("fixtures/single-item-array.php"));
snap!(snapper, multi_item_array, process("fixtures/multi-item-array.php"));
snap!(snapper, nested_array, process("fixtures/nested-array.php"));
snap!(snapper, more_nested_array, process("fixtures/more-nested-array.php"));
snap!(snapper, array_trailing_comma, process("fixtures/array-trailing-comma.php"));
snap!(snapper, legacy_array, process("fixtures/legacy-array.php"));
snap!(snapper, legacy_array_single_item, process("fixtures/legacy-array-single-item.php"));

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