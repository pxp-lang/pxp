use std::ops::Range;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize, Hash)]
pub struct Span {
    pub start: ByteOffset,
    pub end: ByteOffset,
}

impl Span {
    pub fn new(start: ByteOffset, end: ByteOffset) -> Self {
        Self { start, end }
    }

    pub fn missing() -> Self {
        Self::default()
    }

    pub fn flat(offset: ByteOffset) -> Self {
        Self {
            start: offset,
            end: offset,
        }
    }

    pub fn view<'a>(&self, source: &'a [u8]) -> View<'a> {
        View::new(*self, source)
    }

    pub fn to_range(&self) -> Range<ByteOffset> {
        self.start..self.end
    }

    pub fn len(&self) -> ByteOffset {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn combine(start: Span, end: Span) -> Span {
        Span::new(start.start, end.end)
    }

    pub fn join(&self, other: Span) -> Span {
        Span::new(self.start, other.end)
    }

    pub fn maybe_join(&self, other: Option<Span>) -> Span {
        match other {
            Some(other) => self.join(other),
            None => *self,
        }
    }

    pub fn contains_offset(&self, offset: ByteOffset) -> bool {
        offset >= self.start && offset <= self.end
    }

    pub fn is_before_offset(&self, offset: ByteOffset) -> bool {
        self.end < offset
    }

    pub fn is_after_offset(&self, offset: ByteOffset) -> bool {
        self.start > offset
    }
}

pub type ByteOffset = usize;

pub trait Spanned {
    fn span(&self) -> Span;

    fn start_line(&self, source: &[u8]) -> ByteOffset {
        let (line, _) = byte_offset_to_line_and_column(source, self.span().start);
        line
    }

    fn start_column(&self, source: &[u8]) -> ByteOffset {
        let (_, column) = byte_offset_to_line_and_column(source, self.span().start);
        column
    }

    fn end_line(&self, source: &[u8]) -> ByteOffset {
        let (line, _) = byte_offset_to_line_and_column(source, self.span().end);
        line
    }

    fn end_column(&self, source: &[u8]) -> ByteOffset {
        let (_, column) = byte_offset_to_line_and_column(source, self.span().end);
        column
    }
}

impl<T: Spanned> Spanned for Vec<T> {
    fn span(&self) -> Span {
        if self.is_empty() {
            Span::default()
        } else if self.len() == 1 {
            self.first().unwrap().span()
        } else {
            Span::new(
                self.first().unwrap().span().start,
                self.last().unwrap().span().end,
            )
        }
    }
}

impl<T: Spanned> Spanned for Option<T> {
    fn span(&self) -> Span {
        match self {
            Some(t) => t.span(),
            None => Span::default(),
        }
    }
}

impl Spanned for Span {
    fn span(&self) -> Span {
        *self
    }
}

impl<T: Spanned> Spanned for Box<T> {
    fn span(&self) -> Span {
        self.as_ref().span()
    }
}

impl<T: Spanned> Spanned for &T {
    fn span(&self) -> Span {
        (*self).span()
    }
}

impl<T: Spanned> Spanned for &mut T {
    fn span(&self) -> Span {
        (**self).span()
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct View<'a> {
    span: Span,
    source: &'a [u8],
}

impl<'a> View<'a> {
    fn new(span: Span, source: &'a [u8]) -> Self {
        Self { span, source }
    }

    pub fn to_bytes(&self) -> &'a [u8] {
        &self.source[self.span.start..self.span.end]
    }

    pub fn with_previous_line(self) -> Self {
        Self::new(
            line_to_span(self.source, self.span.start_line(self.source).saturating_sub(1)).join(self.span),
            self.source,
        )
    }

    pub fn with_next_line(self) -> Self {
        Self::new(
            self.span.join(line_to_span(self.source, self.span.end_line(self.source).saturating_add(1))),
            self.source,
        )
    }

    pub fn with_n_previous_lines(self, n: usize) -> Self {
        Self::new(
            line_to_span(self.source, self.span.start_line(self.source).saturating_sub(n)).join(self.span),
            self.source,
        )
    }

    pub fn with_n_next_lines(self, n: usize) -> Self {
        Self::new(
            self.span.join(line_to_span(self.source, self.span.end_line(self.source).saturating_add(n))),
            self.source,
        )
    }

    pub fn to_span(&self) -> Span {
        self.span
    }
}

impl<'a> Spanned for View<'a> {
    fn span(&self) -> Span {
        self.to_span()
    }
}

fn line_to_span(source: &[u8], line: usize) -> Span {
    let start = line_to_byte_offset(source, line);
    let end = line_to_byte_offset(source, line + 1);

    Span::new(start, end)
}

fn line_to_byte_offset(source: &[u8], line: usize) -> ByteOffset {
    let mut offset = 0;
    let mut current_line = 0;

    for (i, c) in source.iter().enumerate() {
        if current_line == line {
            return offset;
        }

        if c == &b'\n' {
            current_line += 1;
        }

        offset = i;
    }

    offset
}

fn byte_offset_to_line_and_column(source: &[u8], offset: ByteOffset) -> (usize, usize) {
    // Line and column numbers are 0-based in the LSP spec.
    let mut line = 0;
    let mut column = 0;

    for i in source.iter().take(offset) {
        if i == &b'\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }

    (line, column)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestElement {
        span: Span,
    }

    impl Spanned for TestElement {
        fn span(&self) -> Span {
            self.span
        }
    }

    #[test]
    fn it_returns_the_correct_span() {
        let element = TestElement {
            span: Span::new(0, 5),
        };

        assert_eq!(element.span().start, 0);
        assert_eq!(element.span().end, 5);
    }

    #[test]
    fn it_returns_the_correct_span_for_vec() {
        let element1 = TestElement {
            span: Span::new(0, 5),
        };

        let element2 = TestElement {
            span: Span::new(5, 10),
        };

        let elements = vec![element1, element2];

        assert_eq!(elements.span().start, 0);
        assert_eq!(elements.span().end, 10);
    }

    #[test]
    fn it_returns_the_correct_span_for_option() {
        let element = TestElement {
            span: Span::new(0, 5),
        };

        let option = Some(element);

        assert_eq!(option.span().start, 0);
        assert_eq!(option.span().end, 5);
    }

    #[test]
    fn it_returns_the_correct_span_for_option_none() {
        let option: Option<TestElement> = None;

        assert_eq!(option.span().start, 0);
        assert_eq!(option.span().end, 0);
    }

    #[test]
    fn it_returns_the_correct_span_for_box() {
        let element = TestElement {
            span: Span::new(0, 5),
        };

        let boxed = Box::new(element);

        assert_eq!(boxed.span().start, 0);
        assert_eq!(boxed.span().end, 5);
    }

    #[test]
    fn it_returns_the_correct_span_for_ref() {
        let element = TestElement {
            span: Span::new(0, 5),
        };

        let reference = &element;

        assert_eq!(reference.span().start, 0);
        assert_eq!(reference.span().end, 5);
    }

    #[test]
    fn it_returns_the_correct_span_for_ref_mut() {
        let mut element = TestElement {
            span: Span::new(0, 5),
        };

        let reference = &mut element;

        assert_eq!(reference.span().start, 0);
        assert_eq!(reference.span().end, 5);
    }

    #[test]
    fn it_returns_the_correct_start_line() {
        let element = TestElement {
            span: Span::new(0, 5),
        };

        let source = b"hello\nworld\n";

        assert_eq!(element.start_line(source), 0);
    }

    #[test]
    fn it_returns_the_correct_start_column() {
        let element = TestElement {
            span: Span::new(0, 5),
        };

        let source = b"hello\nworld\n";

        assert_eq!(element.start_column(source), 0);
    }

    #[test]
    fn it_returns_the_correct_end_line() {
        let element = TestElement {
            span: Span::new(0, 5),
        };

        let source = b"hello\nworld\n";

        assert_eq!(element.end_line(source), 0);
    }

    #[test]
    fn it_returns_the_correct_end_column() {
        let element = TestElement {
            span: Span::new(0, 5),
        };

        let source = b"hello\nworld\n";

        assert_eq!(element.end_column(source), 5);
    }

    #[test]
    fn test_byte_offset_to_line_and_column() {
        let source = b"hello\nworld\n";

        assert_eq!(byte_offset_to_line_and_column(source, 0), (0, 0));
        assert_eq!(byte_offset_to_line_and_column(source, 1), (0, 1));
        assert_eq!(byte_offset_to_line_and_column(source, 5), (0, 5));
        assert_eq!(byte_offset_to_line_and_column(source, 6), (1, 1));
        assert_eq!(byte_offset_to_line_and_column(source, 11), (1, 6));
    }

    #[test]
    fn test_to_range() {
        let span = Span::new(0, 5);

        assert_eq!(span.to_range(), 0..5);
    }

    #[test]
    fn test_view() {
        let source = b"hello\nworld\n";
        let span = Span::new(0, 5);

        assert_eq!(span.view(source).to_bytes(), b"hello");
        assert_eq!(span.view(source).with_next_line().to_bytes(), b"hello\nworld");

        let span = Span::new(6, 11);

        assert_eq!(span.view(source).to_bytes(), b"world");
        assert_eq!(span.view(source).with_previous_line().to_bytes(), b"hello\nworld");

        let source = b"hello\nworld\nfoo\nbar\nbaz\n";
        let span = Span::new(6, 11);

        assert_eq!(span.view(source).with_n_previous_lines(2).to_bytes(), b"hello\nworld");
        assert_eq!(span.view(source).to_span(), span);
        assert_eq!(span.view(source).with_n_next_lines(2).to_bytes(), b"world\nfoo\nbar");
        assert_eq!(span.view(source).with_n_next_lines(2).to_span(), Span::new(6, 19));
    }
}
