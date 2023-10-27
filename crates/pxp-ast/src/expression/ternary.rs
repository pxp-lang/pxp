use crate::Expression;

#[derive(Debug, Clone)]
pub struct TernaryExpression {
    pub condition: Box<Expression>,
    pub then: Box<Expression>,
    pub r#else: Box<Expression>, // `baz()`
}