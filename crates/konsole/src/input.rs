use std::{any::Any, collections::HashMap};

#[derive(Debug, Clone)]
pub struct Input {
    arguments: HashMap<String, String>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            arguments: HashMap::new(),
        }
    }

    pub(crate) fn insert_argument(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.arguments.insert(name.into(), value.into());
    }

    pub fn argument<T: Parseable>(&self, name: impl Into<String>) -> Option<T> {
        let name = name.into();
        let value = self.arguments.get(&name)?;

        T::parse(value)
    }
}

pub trait Parseable {
    fn parse(value: &str) -> Option<Self> where Self: Sized;
}

impl Parseable for String {
    fn parse(value: &str) -> Option<Self> {
        Some(value.to_string())
    }
}

impl Parseable for i32 {
    fn parse(value: &str) -> Option<Self> {
        value.parse().ok()
    }
}

impl Parseable for i64 {
    fn parse(value: &str) -> Option<Self> {
        value.parse().ok()
    }
}

impl Parseable for f32 {
    fn parse(value: &str) -> Option<Self> {
        value.parse().ok()
    }
}

impl Parseable for f64 {
    fn parse(value: &str) -> Option<Self> {
        value.parse().ok()
    }
}

impl Parseable for bool {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }
}

impl Parseable for () {
    fn parse(_: &str) -> Option<Self> {
        Some(())
    }
}