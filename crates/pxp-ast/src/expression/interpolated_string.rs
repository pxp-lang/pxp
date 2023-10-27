use crate::StringPart;

#[derive(Debug, Clone)]
pub struct InterpolatedStringExpression {
    pub parts: Vec<StringPart>,
}