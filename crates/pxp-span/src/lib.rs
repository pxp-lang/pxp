#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub position: usize,
}

impl Span {
    pub fn new(line: usize, column: usize, position: usize) -> Self {
        Self {
            line,
            column,
            position,
        }
    }
}
