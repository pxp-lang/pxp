use konsole::{application::Application, command::Command};

fn main() {
    Application::new("Optional Argument (Example)")
        .version("0.0.1-example")
        .command(
            Command::new("optional")
                .argument("name", true)
                .handle(|input, output| {
                    match input.argument::<String>("name") {
                        Some(name) => {
                            output.success(format!("Your name is {}", name));
                        }
                        None => {
                            output.warning("No name provided.");
                        }
                    }
                })
        )
        .run();
}