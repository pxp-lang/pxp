use pxp_span::Span;

#[derive(Debug, Clone)]
pub struct ConstExpression {
    pub kind: ConstExpressionKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ConstExpressionKind {
    
}