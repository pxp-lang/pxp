# Name Resolver

This crate provides a single `NameResolvingVisitor` that takes in a `&mut [Statement]` and mutates `SimpleIdentifier` nodes **in place**, replacing the given name with a fully-resolved name where applicable.