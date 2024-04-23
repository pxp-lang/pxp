# Lexer

This crate contains all of the code required to convert a string of PHP or PXP code into a set of tokens.

## Overview

This code takes in a string of PHP or PXP code and produces a list of named tokens.

It differs from PHP's own lexer / tokeniser due to the fact that it doesn't use the same token names or IDs and actually has a more extensive list of tokens for things such as `true`, `false`, `null`, `self`, `parent`, etc.

For more information about the tokens themselves, consult the [Token](/crates/pxp-token) crate.

## Performance

Rudimentary benchmarks suggest that this crate is ~25% faster than PHP's internal lexer, provided by the `token_get_all()` or `PhpToken::tokenize()` method.

This benchmark isn't incredibly fair, since our lexer doesn't produce an identical set of tokens, but the performance difference is still good.