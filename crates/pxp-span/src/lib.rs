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

    pub fn contains(&self, offset: ByteOffset) -> bool {
        self.start <= offset && offset <= self.end
    }
}

pub type ByteOffset = usize;

pub trait Spanned {
    fn span(&self) -> Span;
}

impl Spanned for Span {
    fn span(&self) -> Span {
        *self
    }
}

impl<T: Spanned> Spanned for &T {
    fn span(&self) -> Span {
        (*self).span()
    }
}

impl<T: Spanned> Spanned for Box<T> {
    fn span(&self) -> Span {
        self.as_ref().span()
    }
}

impl<T: Spanned> Spanned for Vec<T> {
    fn span(&self) -> Span {
        if let Some(first) = self.first() {
            let last = self.last().unwrap();
            Span::new(first.span().start, last.span().end)
        } else {
            Span::default()
        }
    }
}

impl<T: Spanned> Spanned for Option<T> {
    fn span(&self) -> Span {
        self.as_ref().map_or(Span::default(), |t| t.span())
    }
}