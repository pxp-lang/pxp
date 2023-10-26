use pxp_span::{Span, HasSpan};

#[derive(Debug, Clone)]
pub struct Expression {
    kind: ExpressionKind,
    span: Span,
}

impl HasSpan for Expression {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {

}