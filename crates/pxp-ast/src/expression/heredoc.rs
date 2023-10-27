use pxp_token::Token;

use crate::StringPart;

#[derive(Debug, Clone)]
pub struct HeredocExpression {
    pub label: Token,
    pub parts: Vec<StringPart>,
}