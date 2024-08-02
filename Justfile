test:
    cargo nextest run

tokenise +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin tokenise --release -- {{args}}

parse +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin parse --release -- {{args}}

node-finder +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin node-finder --release -- {{args}}

generate-ast:
    php ./meta/scripts/generate-ast.php
    cargo fmt --package pxp-ast

generate-visitor:
    php ./meta/scripts/generate-visitor.php
    cargo fmt --package pxp-ast

generate-node-finder:
    php ./meta/scripts/generate-node-finder.php
    cargo fmt --package pxp-node-finder

bench-parser:
    cargo build --release --bin parse
    hyperfine --warmup=1 --runs=1 \
        --command-name="PHP (AST Ext)" "php ./benches/parsing/core.php ./playground/laravel-framework" \
        --command-name="PXP (Release)" "./target/release/parse ./playground/laravel-framework --no-output"

bench-lexer:
    cargo build --release --bin tokenise
    hyperfine --warmup=1 --runs=1 \
        --command-name="PHP (Core)" "php ./benches/lexing/php.php ./playground/laravel-framework" \
        --command-name="PHP (Core + Opcache)" "php -d opcache.enable_cli=1 ./benches/lexing/php.php ./playground/laravel-framework" \
        --command-name="PXP (Release)" "./target/release/tokenise ./playground/laravel-framework --no-output"

bench-indexer:
    cargo build --release --bin index
    hyperfine --warmup=1 --runs=1 \
        --command-name="Indexer (Release)" "./target/release/index ./playground/laravel-framework --no-output"