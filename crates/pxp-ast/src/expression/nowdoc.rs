use pxp_token::Token;

#[derive(Debug, Clone)]
pub struct NowdocExpression {
    pub label: Token,
    pub value: Token,
}