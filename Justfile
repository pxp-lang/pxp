test:
    cargo test --workspace --lib --bins --tests

generate-ast:
    php ./meta/scripts/generate-ast.php
    cargo fmt --package pxp-ast

generate-visitor:
    php ./meta/scripts/generate-visitor.php
    cargo fmt --package pxp-ast
