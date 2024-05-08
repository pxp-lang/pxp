use pxp_span::{ByteOffset, Span};

#[derive(Debug)]
pub struct Source<'a> {
    input: &'a [u8],
    length: usize,
    offset: ByteOffset,
    current_token_start_offset: ByteOffset,
}

impl<'a> Source<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        let length = input.len();

        Self {
            input,
            length,
            offset: 0,
            current_token_start_offset: 0,
        }
    }

    pub fn offset(&self) -> ByteOffset {
        self.offset
    }

    pub fn start_token(&mut self) {
        self.current_token_start_offset = self.offset;
    }

    pub fn span(&self) -> Span {
        Span::new(self.current_token_start_offset, self.offset)
    }

    pub const fn eof(&self) -> bool {
        self.offset >= self.length
    }

    pub fn next(&mut self) {
        self.offset += 1;
    }

    pub fn skip(&mut self, count: usize) {
        for _ in 0..count {
            self.next();
        }
    }

    pub fn read_and_skip(&mut self, count: usize) -> &'a [u8] {
        let (from, until) = self.to_bound(count);

        self.skip(count);

        &self.input[from..until]
    }

    pub fn current(&self) -> Option<&'a u8> {
        if self.offset >= self.length {
            None
        } else {
            Some(&self.input[self.offset])
        }
    }

    pub fn read(&self, n: usize) -> &'a [u8] {
        let (from, until) = self.to_bound(n);

        &self.input[from..until]
    }

    #[inline(always)]
    pub fn read_remaining(&self) -> &'a [u8] {
        &self.input[(if self.offset >= self.length {
            self.length
        } else {
            self.offset
        })..]
    }

    pub fn at(&self, search: &[u8], len: usize) -> bool {
        self.read(len) == search
    }

    pub fn at_case_insensitive(&self, search: &[u8], len: usize) -> bool {
        let (from, until) = self.to_bound(len);

        let slice = &self.input[from..until];

        slice.eq_ignore_ascii_case(search)
    }

    pub fn peek(&self, i: usize, n: usize) -> &'a [u8] {
        let from = self.offset + i;
        if from >= self.length {
            return &self.input[self.length..self.length];
        }

        let mut until = from + n;
        if until >= self.length {
            until = self.length;
        }

        &self.input[from..until]
    }

    pub fn peek_ignoring_whitespace(&self, i: usize, n: usize) -> &'a [u8] {
        let mut i = i;

        loop {
            let c = self.peek(i, 1);

            if c.is_empty() {
                return &[];
            }

            match c[0] {
                b' ' | b'\t' | b'\r' | b'\n' => i += 1,
                _ => break,
            }
        }

        self.peek(i, n)
    }

    const fn to_bound(&self, n: usize) -> (usize, usize) {
        if self.offset >= self.length {
            return (self.length, self.length);
        }

        let mut until = self.offset + n;

        if until >= self.length {
            until = self.length;
        }

        (self.offset, until)
    }

    pub fn span_range(&self, span: Span) -> &'a [u8] {
        &self.input[span.start..span.end]
    }
}
