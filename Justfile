test:
    cargo nextest run

tokenise +args:
    cargo run --package pxp-lexer --bin tokenise --release -- {{args}}

parse +args:
    cargo run --package pxp-parser --bin parse --release -- {{args}}