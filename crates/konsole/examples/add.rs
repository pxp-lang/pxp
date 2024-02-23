use konsole::prelude::*;

fn main() {
    Application::new("Add (Example)")
        .command(
            Command::new("add")
                .argument("a")
                .argument("b")
                .handle(|input, output| {
                    let a = input.argument::<i32>("a").unwrap();
                    let b = input.argument::<i32>("b").unwrap();

                    println!("a + b = {}", a + b);
                })
        )
        .run();
}