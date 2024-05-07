# Either

This crates implements a general-purpose sum type for expressing an "either or" type.

## Usage

```rs
use pxp_either::Either;

struct NewStatement {
    pub name: Either<Name, Expression>,
}

let statement = NewStatement {
    name: Either::Left(Name::new(...)),
};

statement.is_left();       // true
statement.is_right();      // false
statement.is::<Name>::();  // true
```