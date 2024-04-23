# Snappers

Simple snapshot testing library for Rust.

## Usage

Create a `snapper` function that produces a `Snapper` instance.

```rs
use snapper::Snapper;

fn snapper() -> Snapper {
    Snapper::new(
        // This is the path where generated snapshots will be stored.
        format!("{}/__snapshot__", env!("CARGO_MANIFEST_DIR")).into()
    )
}
```

Then generate test cases using the `snap!()` macro, passing through the name of the `snapper` function, the name of the test case (must be a valid Rust function name), as well as the test subject function.

```rs
#[cfg(test)]
mod tests {
    use snapper::{Snapper, snap};

    snap!(snapper, it_can_say_hello_world, say_hello());

    fn say_hello() -> String {
        return format!("Hello, world!");
    }

    fn snapper() -> Snapper {
        Snapper::new(
            // This is the path where generated snapshots will be stored.
            format!("{}/__snapshot__", env!("CARGO_MANIFEST_DIR")).into()
        )
    }
}
```

Now when you run `cargo test`, the snapshot files will be generated and all further test runs will test against the generated snapshot file (if present).