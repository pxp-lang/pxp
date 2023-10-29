# Parser

This crate provide an error-tolerant parser for PHP and PXP code. It relies on information provided by both the [Lexer](../pxp-lexer) and [AST](../pxp-ast) crates to process a stream of tokens and produce an error resilient concrete syntax tree.

The parser itself is a handwritten, recursive-descent parser that produces a series of `Statement` values. 

The error tolerance techniques used are heavily inspired by the [microsoft/tolerant-php-parser](https://github.com/microsoft/tolerant-php-parser/) package. This allows the parser to be used in a variety of situations, including language servers, static analysis and compilation processes.

Much of the code was lifted from my original PHP parser project [php-rust-tools/parser](https://github.com/php-rust-tools/parser), but has been further extended to be fully error tolerant.