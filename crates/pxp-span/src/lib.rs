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

pub trait Spanned {
    fn span(&self) -> Span;

    fn start_line(&self, source: &[u8]) -> usize {
        let (line, _) = byte_offset_to_line_and_column(source, self.span().start);
        line
    }

    fn start_column(&self, source: &[u8]) -> usize {
        let (_, column) = byte_offset_to_line_and_column(source, self.span().start);
        column
    }

    fn end_line(&self, source: &[u8]) -> usize {
        let (line, _) = byte_offset_to_line_and_column(source, self.span().end);
        line
    }

    fn end_column(&self, source: &[u8]) -> usize {
        let (_, column) = byte_offset_to_line_and_column(source, self.span().end);
        column
    }
}

fn byte_offset_to_line_and_column(source: &[u8], offset: ByteOffset) -> (usize, usize) {
    let mut line = 1;
    let mut column = 1;

    for i in 0..offset {
        if source[i] == b'\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }

    (line, column)
}