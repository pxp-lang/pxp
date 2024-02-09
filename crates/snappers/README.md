# Snappers

Snappers is a library that provides a set of utilities for snapshot testing.

The public API of the library is designed to be as simple as possible. You provide a single function that returns some form of "displayable" value (anything that implements `Display`) and when the test executes, it will check to see if the generated output matches the snapshot.

When the test runs for the first time, a snapshot will be generated based on the return value of the function. Snapshot directory needs to be configured before use.

## Usage

```rs
use snappers::snap!;

snap!(it_can_say_hello_world, say_hello("world"));

fn say_hello(name: &str) -> impl Display {
    return format!("Hello, {name}!");
}
```

Run `cargo test` like usual and Snapper will generate a snapshot based on the test case name.