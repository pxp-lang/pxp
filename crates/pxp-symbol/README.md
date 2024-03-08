# Symbol Table

## Singleton API

This crate does provide a static singleton API around `SymbolTable` via the `SymbolTable::the()` method.

Since this method relies on a `static mut` variable inside of a private method, it **is not** thread safe. This method is only used internally where we're not relying on multi-threading.

Any PRs to make it thread-safe are welcome, but since the project does not need it to be thread-safe, it isn't (sorry).

## Notes

1. Symbols start from `1`, since we reserve `0` for internal use (missing & invalid symbols).