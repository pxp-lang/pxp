# Konsole

Konsole is a package for building command-line applications in Rust. It's heavily inspired by [Symfony's Console component] and [Laravel's Artisan component].

## Installation

```sh
cargo add konsole
```

## Usage

```rs
use konsole::prelude::*;

fn main() {
    Application::new()
        .command(
            Command::new("add")
                .argument("a", "The first number.")
                .argument("b", "The second number.")
                .handle(|input, output| {
                    let a = input.argument::<i32>("a")?;
                    let b = input.argument::<i32>("b")?;

                    println!("a + b = {}", a + b);

                    Ok(())
                })
        )
        .run();
}
```