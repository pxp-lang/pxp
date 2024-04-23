test:
    cargo nextest run

tokenise +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin tokenise --release -- {{args}}

parse +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin parse --release -- {{args}}

bench:
    time php ./benches/parsing/nikic.php ./playground/laravel-framework
    cargo build --release --bin parse
    time ./target/release/parse ./playground/laravel-framework --no-output