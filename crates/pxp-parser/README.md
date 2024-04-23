# Parser

This crate contains all of the code required to convert a stream of tokens into a set of AST nodes.

## Overview

The parser takes in a stream of tokens and translates recognisable patterns into AST nodes. It implements a recursive-descent style parser with precedence and associativity rules for parsing expressions.

### Error tolerance

One of the goals of the parser is to be error-tolerant so that it can be used inside of the language server project. The approach to error tolerance is heavily inspired by the excellent [`microsoft/tolerant-php-parser`](https://github.com/microsoft/tolerant-php-parser) package.

Error tolerance support is entirely subjective and will improve over time. The majority of places it is supported is solely based on language server development and the most common places where tolerance is required for autocomplete.

### Performance

Rudimentary benchmarks show that this parser is ~8-9 times faster than the most popular PHP parser [nikic/php-parser](https://github.com/nikic/PHP-Parser).

> This result is of course expected given that `nikic/php-parser` is written purely in PHP and uses a YACC-style parser generator, whereas the parser provided by this project is handwritten and uses Rust.

When compared to PHP's own parser (exposed via `ext-ast`), it is about 40% slower which equates to about 600ms. There are definitely parsing-specific optimisations to be made in the PXP parser, but there are also other things such as Rust vs C performance, memory allocations, etc that affect the performance.