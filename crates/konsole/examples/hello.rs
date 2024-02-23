use konsole::prelude::*;

fn main() {
    Application::new("Hello (Example)")
        .version("0.0.1-example")
        .command(
            Command::new("hello")
                .description("Prints a greeting.")
                .handle(|| {
                    println!("Hello, world!");
                })
        )
        .run();
}