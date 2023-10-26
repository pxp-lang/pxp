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

    pub fn read_remaining(&mut self) -> &'a [u8] {
        let from = self.position;
        self.position = self.length;

        &self.input[from..]
    }

    pub fn read_and_skip_n(&mut self, n: usize) -> &'a [u8] {
        let from = self.position;
        let until = if self.position + n > self.length {
            self.length
        } else {
            self.position + n
        };

        self.skip_n(n);

        &self.input[from..until]
    }

    pub fn read_n(&self, n: usize) -> &'a [u8] {
        let from = self.position;
        let until = if self.position + n > self.length {
            self.length
        } else {
            self.position + n
        };

        &self.input[from..until]
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

    pub fn matches(&self, needle: u8) -> bool {
        !self.is_eof() && self.input[self.position] == needle
    }

    pub fn matches_n(&self, pattern: &[u8], n: usize) -> bool {
        let from = self.position;
        let until = if self.position + n > self.length {
            self.length
        } else {
            self.position + n
        };

        let slice = &self.input[from..until];
        slice.eq_ignore_ascii_case(pattern)
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
        let from = self.position + from;
        if from >= self.length {
            return &self.input[self.length..self.length];
        }

        let mut until = from + until;
        if until >= self.length {
            until = self.length;
        }

        &self.input[from..until]
    }

    pub fn peek_ignoring_whitespace(&self, i: usize, n: usize) -> &'a [u8] {
        let mut i = i;

        loop {
            let c = self.peek_range(i, 1);

            if c.is_empty() {
                return &[];
            }

            match c[0] {
                b' ' | b'\t' | b'\r' | b'\n' => i += 1,
                _ => break,
            }
        }

        self.peek_range(i, n)
    }
}