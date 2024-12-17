test:
    cargo test --lib --bins --tests
    cargo test --lib --bins --tests --features docblocks

tokenise +args:
    RUSTFLAGS=-Awarnings cargo build -q --package pxp-internal --bin tokenise
    ./target/debug/tokenise {{args}}

parse +args:
    RUSTFLAGS=-Awarnings cargo build -q --package pxp-internal --bin parse
    ./target/debug/parse {{args}}

node-finder +args:
    RUSTFLAGS=-Awarnings cargo build -q --package pxp-internal --bin node-finder
    ./target/debug/node-finder {{args}}

infer +args:
    RUSTFLAGS=-Awarnings cargo build -q --package pxp-internal --bin infer
    ./target/debug/infer {{args}}

generate-ast:
    php ./meta/scripts/generate-ast.php
    cargo fmt --package pxp-ast

generate-visitor:
    php ./meta/scripts/generate-visitor.php
    cargo fmt --package pxp-ast
