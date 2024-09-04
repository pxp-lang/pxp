# LSP

This crate provides a more convenient API around the excellent [lsp-server](https://github.com/rust-lang/rust-analyzer/tree/master/lib/lsp-server) crate developed by the `rust-analyzer` team.

The original crate is quite rugged and barebones. It leaves us, the developers, to manually wire up requests, responses and notifications without a clearly defined API or standard.

To combat this problem, I decided to develop a wrapper around the crate that focuses on developer-experience and simplicity.

## Usage

