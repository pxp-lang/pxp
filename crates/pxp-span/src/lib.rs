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

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn combine(start: Span, end: Span) -> Span {
        Span::new(start.start, end.end)
    }

    pub fn contains_offset(&self, offset: ByteOffset) -> bool {
        offset >= self.start && offset < self.end
    }

    pub fn is_before_offset(&self, offset: ByteOffset) -> bool {
        self.end < offset
    }

    pub fn is_after_offset(&self, offset: ByteOffset) -> bool {
        self.start >= offset
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

impl<T: Spanned> Spanned for Vec<T> {
    fn span(&self) -> Span {
        if self.is_empty() {
            Span::default()
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

fn byte_offset_to_line_and_column(source: &[u8], offset: ByteOffset) -> (usize, usize) {
    let mut line = 1;
    let mut column = 1;

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

        assert_eq!(element.start_line(source), 1);
    }

    #[test]
    fn it_returns_the_correct_start_column() {
        let element = TestElement {
            span: Span::new(0, 5),
        };

        let source = b"hello\nworld\n";

        assert_eq!(element.start_column(source), 1);
    }

    #[test]
    fn it_returns_the_correct_end_line() {
        let element = TestElement {
            span: Span::new(0, 5),
        };

        let source = b"hello\nworld\n";

        assert_eq!(element.end_line(source), 1);
    }

    #[test]
    fn it_returns_the_correct_end_column() {
        let element = TestElement {
            span: Span::new(0, 5),
        };

        let source = b"hello\nworld\n";

        assert_eq!(element.end_column(source), 6);
    }

    #[test]
    fn test_byte_offset_to_line_and_column() {
        let source = b"hello\nworld\n";

        assert_eq!(byte_offset_to_line_and_column(source, 0), (1, 1));
        assert_eq!(byte_offset_to_line_and_column(source, 1), (1, 2));
        assert_eq!(byte_offset_to_line_and_column(source, 5), (1, 6));
        assert_eq!(byte_offset_to_line_and_column(source, 6), (2, 1));
        assert_eq!(byte_offset_to_line_and_column(source, 11), (2, 6));
    }
}
