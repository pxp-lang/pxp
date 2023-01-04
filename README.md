# PXP

PXP is a superset of the PHP programming language that provides an extended set of syntax rules and language features to improve developer experience.

## Roadmap

Whilst this repository _is_ public, it is not ready for any form of development or production use. The codebase is constantly being updated with new features and improvements.

Refer to the [Features](#features) section below for a list of planned and supported features.

The list below is a brief overview of what has been done and still needs to be done.

* [x] Initial parser implementation.
* [x] Basic command-line interface for `pxp` binary.
* [x] Transpiler trait.
* [x] AST traverser (needs improvement).
* [ ] Transpilers for initially supported features.
* [ ] AST printer (formatting doesn't matter, for now at least).
* [ ] PHPStan plugin (and other appropriate analysis tools).

## Features

**Short match**

```php
$a = new Foo;

match {
    $a instanceof Foo => ...,
};
```

**Multi-line match arms**

```php
$a = match (foo()) {
    "bar" => {
        // Execute multiple statements...
        return $something;
    }
};
```

> The syntax for returning a value for block is still subject to change.

**Local type aliases**

```php
type number = int|float;

function add(number $a, number $b): number {
    return $a + $b;
}
```

> Type aliases will only work inside of the file where they are defined. There are plans to introduce "use"-able type aliases in the future.

**Range expressions**

```php
$_ = 1..3; // Exclusive range, i.e. [1, 2]
$_ = 1..=3; // Inclusive range, i.e. [1, 2, 3]
$_ = 1..; // Endless range, i.e. [1, ..., ∞]
```

The difference between `range(...)` and PXP's range expression is the return value. PHP's `range(...)` function will return an array, whereas PXP's expression actually returns a `Range` object.

The `Range` object is iterable and can be used in looping structures. It also behaves like a regular array so indexing is possible (you cannot set the value of an index). There are also other methods available on the object (documentation coming soon), which will allow you to interact with a range after creation (change step between items, check if a value is contained within a range).

**Multi-line short closures (with auto capturing)**

```php
function wrap(callable $callable, mixed ...$arguments) {
    return fn () {
        return $callable(...$arguments);
    };
}
```

> Multi-line short closures will auto-capture the surrounding environment **by value**, as specified in the [original RFC](https://wiki.php.net/rfc/auto-capture-closure).

**More coming soon...**

## License

Licensed under:

 * Apache License, Version 2.0
   ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

## Credits

* [Ryan Chandler](https://github.com/ryangjchandler)
* [All contributors](https://github.com/ryangjchandler/php-parser-rs/graphs/contributors)