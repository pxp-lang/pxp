use pxp_span::{Span, HasSpan};

#[derive(Debug, Clone)]
pub struct Statement {
    kind: StatementKind,
    span: Span,
}

impl HasSpan for Statement {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone)]
pub enum StatementKind {

}