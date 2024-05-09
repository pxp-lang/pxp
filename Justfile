test:
    cargo nextest run

tokenise +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin tokenise --release -- {{args}}

parse +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin parse --release -- {{args}}

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