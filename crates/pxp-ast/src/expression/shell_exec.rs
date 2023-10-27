use crate::StringPart;

#[derive(Debug, Clone)]
pub struct ShellExecExpression {
    pub parts: Vec<StringPart>,
}