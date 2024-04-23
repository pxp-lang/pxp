test:
    cargo nextest run

tokenise +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin tokenise --release -- {{args}}

parse +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin parse --release -- {{args}}

bench:
    cargo build --release --bin parse
    hyperfine --warmup=1 --runs=1 \
        --command-name="PHP (Nikic)" "php ./benches/parsing/nikic.php ./playground/laravel-framework" \
        --command-name="PHP (AST Ext)" "php ./benches/parsing/core.php ./playground/laravel-framework" \
        --command-name="PXP (Release)" "./target/release/parse ./playground/laravel-framework --no-output"