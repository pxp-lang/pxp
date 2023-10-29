#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    #[inline]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn size(&self) -> usize {
        self.end - self.start
    }

    pub fn text<'a>(&self, text: &'a [u8]) -> &'a [u8] {
        &text[self.start..self.end]
    }

    pub fn with_start_as_end(&self) -> Self {
        Self::new(self.start, self.start)
    }
}

impl From<(usize, usize)> for Span {
    fn from((start, end): (usize, usize)) -> Self {
        Self::new(start, end)
    }
}

pub trait HasSpan {
    fn span(&self) -> Span;
}

#[cfg(test)]
mod tests {
    use super::Span;

    #[test]
    fn it_can_be_created() {
        let span = Span::new(0, 1);

        assert_eq!(span.start, 0);
        assert_eq!(span.end, 1);
    }

    #[test]
    fn it_can_calculate_size() {
        let span = Span::new(0, 5);

        assert_eq!(span.size(), 5);
    }

    #[test]
    fn it_can_return_span_text_from_source_text() {
        let span = Span::new(0, 5);
        let text = b"Hello, world!";

        assert_eq!(span.text(text), b"Hello");
    }

    #[test]
    fn it_can_be_created_from_tuple() {
        let span = Span::from((0, 5));

        assert_eq!(span.start, 0);
        assert_eq!(span.end, 5);
    }
}