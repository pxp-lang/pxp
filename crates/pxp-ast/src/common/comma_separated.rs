use std::{vec::IntoIter, slice::{Iter, IterMut}};

#[derive(Debug, Clone)]
pub struct CommaSeparated<T> {
    pub items: Vec<T>,
}

impl<T> CommaSeparated<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.items.iter_mut()
    }
}

impl<T> Default for CommaSeparated<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for CommaSeparated<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}