use pxp_token::Token;

use crate::Expression;

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub kind: InfixOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum InfixOperator {
    Add(Token),
    Subtract(Token),
    Multiply(Token),
    Divide(Token),
    Modulo(Token),
    Pow(Token),
    Assign(Token),
    AddAssign(Token),
    SubtractAssign(Token),
    MultiplyAssign(Token),
    DivideAssign(Token),
    ModuloAssign(Token),
    PowAssign(Token),
    BitwiseAndAssign(Token),
    BitwiseOrAssign(Token),
    BitwiseXorAssign(Token),
    LeftShiftAssign(Token),
    RightShiftAssign(Token),
    NullCoalesceAssign(Token),
    NullCoalesce(Token),
    Concat(Token),
    ConcatAssign(Token),
    BitwiseAnd(Token),
    BitwiseOr(Token),
    BitwiseXor(Token),
    LeftShift(Token),
    RightShift(Token),
    Equal(Token),
    Identical(Token),
    NotEqual(Token),
    NotIdentical(Token),
    LessThan(Token),
    LessThanOrEqual(Token),
    GreaterThan(Token),
    GreaterThanOrEqual(Token),
    Spaceship(Token),
    And(Token),
    Or(Token),
    Instanceof(Token),
}