test:
    cargo test --lib --bins --tests
    cargo test --lib --bins --tests --features docblocks

tokenise +args:
    RUSTFLAGS=-Awarnings cargo build -q --package pxp-internal --bin tokenise --release
    ./target/release/tokenise {{args}}

bench-tokenise +args:
    RUSTFLAGS=-Awarnings cargo build -q --package pxp-internal --bin tokenise --release
    hyperfine --warmup=1 --runs=3 \
        --command-name="PXP (Release)" "./target/release/tokenise {{args}}"

parse +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-internal --bin parse -- {{args}}

node-finder +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-internal --bin node-finder -- {{args}}

infer +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-internal --bin infer -- {{args}}

generate-ast:
    php ./meta/scripts/generate-ast.php
    cargo fmt --package pxp-ast

generate-visitor:
    php ./meta/scripts/generate-visitor.php
    cargo fmt --package pxp-ast

bench-parser +args:
    cargo build --release --bin parse
    hyperfine --warmup=1 --runs=1 \
        --command-name="PHP (AST Ext)" "php ./benches/parsing/core.php {{args}}" \
        --command-name="PXP (Release)" "./target/release/parse {{args}} --no-output"

bench-lexer +args:
    cargo build --release --bin tokenise
    hyperfine --warmup=1 --runs=1 \
        --command-name="PHP (Core)" "php ./benches/lexing/php.php {{args}}" \
        --command-name="PHP (Core + Opcache)" "php -d opcache.enable_cli=1 ./benches/lexing/php.php {{args}}" \
        --command-name="PXP (Release)" "./target/release/tokenise {{args}} --no-output"

bench-indexer +args:
    cargo build --release --bin index
    hyperfine --warmup=1 --runs=1 \
        --command-name="Indexer (Release)" "./target/release/index {{args}} --no-output"

bench-visitor +args:
    cargo build --release --bin visit
    hyperfine --warmup=1 --runs=1 \
        --command-name="Visitor (Release)" "./target/release/visit {{args}} --no-output"

bench-node-visitor +args:
    cargo build --release --bin node-visit
    hyperfine --warmup=1 --runs=1 \
        --command-name="Node Visitor (Release)" "./target/release/node-visit {{args}} --no-output"
