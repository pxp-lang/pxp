# Standard Library Indexes

This crate is responsible for generating indexes for PHP's own standard library functions, as well as any extensions that we have stubs for.

Indexes can be serialised and stored on disk, so we do that for each PHP version and then provide them as constants in the crate, as well as functions for getting the deserialised indexes.

All of the work is done with a build script. This means any dependencies we pull in for generating the indexes aren't then required when using the indexes.