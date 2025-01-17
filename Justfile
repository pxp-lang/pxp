test:
    cargo test --workspace --lib --bins --tests

ast:
    php ./meta/generate-ast.php
    php ./meta/generate-visitor.php
    cargo fmt --package pxp-ast

stubs:
    php ./meta/generate-stubs.php

meta: ast stubs
