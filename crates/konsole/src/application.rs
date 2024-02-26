use std::{env, process::exit};
use colored::*;

use crate::{command::Command, prelude::{Input, Output}};

#[derive(Debug, Clone)]
pub struct Application {
    name: String,
    version: Option<String>,
    commands: Vec<Command>,
}

impl Application {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: None,
            commands: Vec::new(),
        }
    }

    pub fn version(&mut self, version: impl Into<String>) -> &mut Self {
        self.version = Some(version.into());
        self
    }

    pub fn command(&mut self, command: Command) -> &mut Self {
        if &command.name == "help" {
            panic!("`help` is a reserved command name.");
        }

        self.commands.push(command);
        self
    }

    fn help(&self) {
        print!("{} ", self.name);

        if let Some(version) = &self.version {
            println!("{}", version.green());
        } else {
            println!();
        }

        println!();
        println!("{}", "Usage:".yellow());
        println!("  command [options] [arguments]");
        println!();

        println!("{}", "Options:".yellow());
        println!("  {} Display this help message.", "-h, --help".green());
        println!("  {} Display the current version.", "-V, --version".green());
        println!();

        println!("{}", "Available commands:".yellow());
        println!("  {} Display this help message.", "help".green());

        for command in &self.commands {
            println!("  {} {}", command.name.green(), command.description.as_ref().unwrap_or(&String::new()));
        }
    }

    pub fn run(&self) {
        let args = env::args().skip(1).collect::<Vec<_>>();
        let output = Output::new();

        if args.is_empty() {
            self.help();
            exit(0);
        }

        let (name, arguments_and_options) = args.split_at(1);

        if name[0] == "help" {
            self.help();
            exit(0);
        }

        let command = match self.commands.iter().find(|command| command.name == name[0]) {
            Some(command) => command,
            None => {
                output.error(format!(r#"Command "{}" is not found."#, name[0]));
                exit(1);
            }
        };

        if arguments_and_options.contains(&"-h".to_string()) || arguments_and_options.contains(&"--help".to_string()) {
            command.help();
            exit(0);
        }

        let mut input = Input::new();
        let offset = 0;

        for (i, (name, optional)) in command.arguments.iter() {
            let value = match arguments_and_options.get(i + offset) {
                Some(value) => value,
                None => {
                    if *optional {
                        continue;
                    }

                    output.error(format!(r#"Argument "{}" is required."#, name));
                    exit(1);
                }
            };

            input.insert_argument(name, value);
        }

        if let Some(handler) = &command.handler {
            handler(input, output);
            return;
        }
    }
}