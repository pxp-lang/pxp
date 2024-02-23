use colored::*;

pub struct Output;

impl Output {
    pub fn new() -> Self {
        Self
    }

    pub fn info(&self, message: impl Into<String>) {
        println!("{}", message.into().blue());
    }

    pub fn success(&self, message: impl Into<String>) {
        println!("{}", message.into().green());
    }

    pub fn warning(&self, message: impl Into<String>) {
        println!("{}", message.into().yellow());
    }

    pub fn error(&self, message: impl Into<String>) {
        println!("{}", message.into().red().bold());
    }
}