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
}