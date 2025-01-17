test:
    cargo test --workspace --lib --bins --tests

generate-ast:
    php ./meta/generate-ast.php
    php ./meta/generate-visitor.php
    cargo fmt --package pxp-ast

generate-stubs:
    php ./meta/generate-stubs.php
