use crate::Expression;

#[derive(Debug, Clone)]
pub struct MatchExpression {
    pub subject: Box<Expression>,
    pub default: Option<Box<DefaultMatchArm>>,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub conditions: Vec<Expression>,
    pub body: Expression,
}

#[derive(Debug, Clone)]
pub struct DefaultMatchArm {
    pub body: Expression,
}