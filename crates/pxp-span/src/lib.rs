use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize, Hash)]
pub struct Span {
    pub start: ByteOffset,
    pub end: ByteOffset,
}

impl Span {
    pub fn new(start: ByteOffset, end: ByteOffset) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub type ByteOffset = usize;