test:
    cargo nextest run

tokenise +args:
    cargo run --package pxp-lexer --bin tokenise --release -- {{args}}