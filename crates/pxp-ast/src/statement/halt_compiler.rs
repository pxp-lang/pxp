use pxp_token::Token;

#[derive(Debug, Clone)]
pub struct HaltCompilerStatement {
    pub content: Option<Token>,
}