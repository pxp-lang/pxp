use indexmap::IndexMap;
use colored::*;

use crate::prelude::{Input, Output};

pub type CommandHandler = fn (Input, Output) -> ();

#[derive(Debug, Default, Clone)]
pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub arguments: IndexMap<String, ()>,
    pub handler: Option<CommandHandler>,
}

impl Command {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            arguments: IndexMap::default(),
            handler: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn argument(mut self, name: impl Into<String>) -> Self {
        self.arguments.insert(name.into(), ());
        self
    }

    pub(crate) fn help(&self) {
        if let Some(description) = &self.description {
            println!("{}", "Description:".yellow());
            println!("  {}", description);
            println!();
        }

        println!("{}", "Usage:".yellow());
        println!("  {} [options] [--] {}", self.name.bold(), self.arguments.keys().map(|key| format!("<{}>", key)).collect::<Vec<String>>().join(" "));
        println!();

        if !self.arguments.is_empty() {
            println!("{}", "Arguments:".yellow());

            for (name, _) in &self.arguments {
                println!("  {}", name.green());
            }

            println!();
        }

        println!("{}", "Options:".yellow());
        println!("  {} Display this help message.", "-h, --help".green());
    }

    pub fn handle(mut self, handler: CommandHandler) -> Self {
        self.handler = Some(handler);
        self
    }
}