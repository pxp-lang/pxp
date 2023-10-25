#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Hash, Default)]
pub struct Source<'a> {
    input: &'a [u8],
    length: usize,
    position: usize,
}

impl<'a> Source<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        let length = input.len();

        Self { input, length, position: 0 }
    }

    pub const fn position(&self) -> usize {
        self.position
    }

    pub const fn length(&self) -> usize {
        self.length
    }

    pub fn is_eof(&self) -> bool {
        self.position >= self.length
    }

    pub fn next(&mut self) {
        self.position += 1;
    }

    pub fn current(&self) -> Option<&'a u8> {
        if self.is_eof() {
            None
        } else {
            Some(&self.input[self.position])
        }
    }

    pub fn skip_n(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }

    pub fn skip_whitespace(&mut self) {
        while !self.is_eof() {
            match self.current() {
                Some(b) if b.is_ascii_whitespace() => self.next(),
                _ => break,
            }
        }
    }

    pub fn peek(&self) -> Option<&'a u8> {
        if self.position + 1 >= self.length {
            None
        } else {
            Some(&self.input[self.position + 1])
        }
    }

    pub fn peek_n(&self, n: usize) -> &'a [u8] {
        let from = self.position;
        let until = if self.position + n > self.length {
            self.length
        } else {
            self.position + n
        };

        self.peek_range(from, until)
    }

    pub fn peek_range(&self, from: usize, until: usize) -> &'a [u8] {
        &self.input[from..until]
    }
}