test:
    cargo test --workspace --lib --bins --tests

generate-ast:
    php ./meta/scripts/generate-ast.php
    php ./meta/scripts/generate-visitor.php
    cargo fmt --package pxp-ast
