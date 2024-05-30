use crate::{AssignmentOperationExpression, Expression};
use pxp_span::Span;

impl AssignmentOperationExpression {
    pub fn left(&self) -> &Expression {
        match self {
            AssignmentOperationExpression::Assign { left, .. } => left.as_ref(),
            AssignmentOperationExpression::Addition { left, .. } => left.as_ref(),
            AssignmentOperationExpression::Subtraction { left, .. } => left.as_ref(),
            AssignmentOperationExpression::Multiplication { left, .. } => left.as_ref(),
            AssignmentOperationExpression::Division { left, .. } => left.as_ref(),
            AssignmentOperationExpression::Modulo { left, .. } => left.as_ref(),
            AssignmentOperationExpression::Exponentiation { left, .. } => left.as_ref(),
            AssignmentOperationExpression::Concat { left, .. } => left.as_ref(),
            AssignmentOperationExpression::BitwiseAnd { left, .. } => left.as_ref(),
            AssignmentOperationExpression::BitwiseOr { left, .. } => left.as_ref(),
            AssignmentOperationExpression::BitwiseXor { left, .. } => left.as_ref(),
            AssignmentOperationExpression::LeftShift { left, .. } => left.as_ref(),
            AssignmentOperationExpression::RightShift { left, .. } => left.as_ref(),
            AssignmentOperationExpression::Coalesce { left, .. } => left.as_ref(),
        }
    }

    pub fn right(&self) -> &Expression {
        match self {
            AssignmentOperationExpression::Assign { right, .. } => right.as_ref(),
            AssignmentOperationExpression::Addition { right, .. } => right.as_ref(),
            AssignmentOperationExpression::Subtraction { right, .. } => right.as_ref(),
            AssignmentOperationExpression::Multiplication { right, .. } => right.as_ref(),
            AssignmentOperationExpression::Division { right, .. } => right.as_ref(),
            AssignmentOperationExpression::Modulo { right, .. } => right.as_ref(),
            AssignmentOperationExpression::Exponentiation { right, .. } => right.as_ref(),
            AssignmentOperationExpression::Concat { right, .. } => right.as_ref(),
            AssignmentOperationExpression::BitwiseAnd { right, .. } => right.as_ref(),
            AssignmentOperationExpression::BitwiseOr { right, .. } => right.as_ref(),
            AssignmentOperationExpression::BitwiseXor { right, .. } => right.as_ref(),
            AssignmentOperationExpression::LeftShift { right, .. } => right.as_ref(),
            AssignmentOperationExpression::RightShift { right, .. } => right.as_ref(),
            AssignmentOperationExpression::Coalesce { right, .. } => right.as_ref(),
        }
    }

    pub fn operator(&self) -> &Span {
        match self {
            AssignmentOperationExpression::Assign { equals, .. } => equals,
            AssignmentOperationExpression::Addition { plus_equals, .. } => plus_equals,
            AssignmentOperationExpression::Subtraction { minus_equals, .. } => minus_equals,
            AssignmentOperationExpression::Multiplication {
                asterisk_equals, ..
            } => asterisk_equals,
            AssignmentOperationExpression::Division { slash_equals, .. } => slash_equals,
            AssignmentOperationExpression::Modulo { percent_equals, .. } => percent_equals,
            AssignmentOperationExpression::Exponentiation { pow_equals, .. } => pow_equals,
            AssignmentOperationExpression::Concat { dot_equals, .. } => dot_equals,
            AssignmentOperationExpression::BitwiseAnd {
                ampersand_equals, ..
            } => ampersand_equals,
            AssignmentOperationExpression::BitwiseOr { pipe_equals, .. } => pipe_equals,
            AssignmentOperationExpression::BitwiseXor { caret_equals, .. } => caret_equals,
            AssignmentOperationExpression::LeftShift {
                left_shift_equals, ..
            } => left_shift_equals,
            AssignmentOperationExpression::RightShift {
                right_shift_equals, ..
            } => right_shift_equals,
            AssignmentOperationExpression::Coalesce {
                coalesce_equals, ..
            } => coalesce_equals,
        }
    }
}
