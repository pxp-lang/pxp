# ByteString

This crate contains a set of helper structures for representing and accurately printing byte-sequences.

## Overview

Since PHP code is not required to be valid UTF-8, we can't use Rust's regular `String` and `&str` values to represent identifiers, variables, etc. Instead, we have to operate on sequences of raw bytes (`u8` values).

The painful part of this is that Rust doesn't have any "clean" ways to print or debug these values, so we instead have two helper structures for owned `Vec<u8>` sequences and borrowed `&[u8]` sequences:
* `ByteString`
* `ByteStr`

Both of these structures can be created from a sequence of bytes and in nearly all cases treated as their underlying data types.