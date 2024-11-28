#![no_main]

use libfuzzer_sys::fuzz_target;
use pxp_ast::node::Node;

fuzz_target!(|data: &[u8]| {
    let Ok(statements) = pxp_parser::parse(data) else { return };

    for mut statement in statements {
        for _node in statement.children() {
        }
    }
});
